use oxc_allocator::Allocator;
use oxc_benchmark::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use oxc_codegen::{CodeGenerator, CodegenReturn};
use oxc_parser::Parser;
use oxc_sourcemap::serialize_sourcemap_mappings;
use oxc_span::SourceType;
use oxc_tasks_common::TestFiles;

#[allow(clippy::cast_possible_truncation)]
fn bench_sourcemap(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("sourcemap_vlq");

    for file in TestFiles::complicated_one(1).files() {
        let id = BenchmarkId::from_parameter(&file.file_name);
        let source_type = SourceType::from_path(&file.file_name).unwrap();
        group.bench_with_input(id, &file.source_text, |b, source_text| {
            let allocator = Allocator::default();
            let ret = Parser::new(&allocator, source_text, source_type).parse();

            let CodegenReturn { source_map, .. } = CodeGenerator::new()
                .enable_source_map(file.file_name.as_str(), source_text)
                .build(&ret.program);
            let source_map = black_box(source_map.unwrap());

            b.iter(|| black_box(serialize_sourcemap_mappings(&source_map)));
        });
    }

    group.finish();
}

criterion_group!(sourcemap, bench_sourcemap);
criterion_main!(sourcemap);
