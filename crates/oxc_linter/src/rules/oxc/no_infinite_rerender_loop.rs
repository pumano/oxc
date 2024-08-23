#[allow(clippy::wildcard_imports)]
use oxc_ast::ast::*;
use oxc_ast::AstKind;
use oxc_macros::declare_oxc_lint;
use oxc_semantic::ScopeId;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Default, Clone)]
pub struct NoInfiniteRerenderLoop;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    NoInfiniteRerenderLoop,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details

    pending  // TODO: describe fix capabilities. Remove if no fix can be done,
             // keep at 'pending' if you think one could be added but don't know how.
             // Options are 'fix', 'fix_dangerous', 'suggestion', and 'conditional_fix_suggestion'
);

impl Rule for NoInfiniteRerenderLoop {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some((cb, deps)) = use_effect_callback_and_deps(call) else {
            return;
        };
        let used_in_scope = node.scope_id();
        if used_in_scope == ctx.scopes().root_scope_id() {
            return;
        }

        for (state, set_state) in iter_state_dependencies(ctx, deps, used_in_scope) {
            println!("[{}, {}]", state.name, set_state.name);
        }
        todo!()
    }
}

/// Get the callback function and dependency array in an `useEffect` hook.
///
/// Returns [`None`] if `expr` is not a call to `useEffect` or if no dependency
/// array is present.
fn use_effect_callback_and_deps<'a, 'c>(
    expr: &'c CallExpression<'a>,
) -> Option<(/* cb */ &'c Expression<'a>, /* deps */ &'c ArrayExpression<'a>)> {
    let name = expr.callee_name()?;
    if name != "useEffect" {
        return None;
    }
    if expr.arguments.len() != 2 {
        return None;
    }
    let cb = &expr.arguments[0].as_expression()?;
    let deps = &expr.arguments[1].as_expression()?;

    match (cb.get_inner_expression(), deps.get_inner_expression()) {
        (
            Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_),
            Expression::ArrayExpression(deps),
        ) => Some((cb, deps.as_ref())),
        _ => None,
    }
}

///
/// Returns an iterator over `[state, setState]` bindings, where `state` is used
/// in an `useEffect` dependency array.
///
/// - `ctx`: lint contenxt
/// - `deps`: the dependency array
/// - `component_scope`: the scope id where the `useEffect` call was used. This
/// will either be a component body or the body of a custom hook function.
fn iter_state_dependencies<'a, 'b>(
    ctx: &'b LintContext<'a>,
    deps: &'b ArrayExpression<'a>,
    component_scope: ScopeId,
) -> impl Iterator<
    Item = (
        /* state */ &'b BindingIdentifier<'a>,
        /* setState */ &'b BindingIdentifier<'a>,
    ),
> + 'b {
    // get identifiers and the 'leftmost' object in member expressions
    // e.g. [foo, bar.baz, 'not an identifier', ...spread] -> [foo, bar]
    deps.elements
        .iter()
        .filter_map(|el| el.as_expression())
        .filter_map(get_base_object)
        .filter_map(move |dep| get_state_dependency(ctx, dep, component_scope))
}

fn get_state_dependency<'a, 'b>(
    ctx: &LintContext<'a>,
    dep: &'b IdentifierReference<'a>,
    component_scope: ScopeId,
) -> Option<(
    /* state */ &'b BindingIdentifier<'a>,
    /* setState */ &'b BindingIdentifier<'a>,
)> {
    // resolve the referenced symbol
    let symbol_id = {
        let reference_id = dep.reference_id()?;
        let reference = ctx.symbols().get_reference(reference_id);
        reference.symbol_id()?
    };

    // we're only looking for dependencies coming from useState or other
    // similar hooks, which will be declared within the component (and thus
    // have the same scope as the useEffect hook call)
    let scope_id = ctx.symbols().get_scope_id(symbol_id);
    if scope_id != component_scope {
        return None;
    }

    // let [...] = unknownCall();
    let (declaration, init) = {
        let declaration_id = ctx.symbols().get_declaration(symbol_id);
        let declaration_node = ctx.nodes().get_node(declaration_id);
        let AstKind::VariableDeclarator(decl) = declaration_node.kind() else {
            return None;
        };
        let BindingPatternKind::ArrayPattern(arr) = &decl.id.kind else {
            return None;
        };
        let Some(Expression::CallExpression(init)) = decl.init.as_ref() else {
            return None;
        };
        (arr.as_ref(), init)
    };

    // TODO: check useReducer or other similar hooks
    if init.callee_name().map_or(true, |name| name != "useState")
        || declaration.elements.len() != 2
        || declaration.rest.is_some()
    {
        return None;
    }
    let state = get_id_at(declaration, 0)?;
    let set_state = get_id_at(declaration, 1)?;

    // Skip cases where setState is the dependency, since calling it won't
    // re-trigger the effect
    if state.symbol_id.get().map_or(true, |state_id| state_id != symbol_id) {
        return None;
    }

    Some((state, set_state))
}

fn get_id_at<'a, 'arr>(
    array: &'arr ArrayPattern<'a>,
    index: usize,
) -> Option<&'arr BindingIdentifier<'a>> {
    array.elements[index].as_ref().and_then(BindingPattern::get_binding_identifier)
}
fn get_base_object<'a, 'c>(expr: &'c Expression<'a>) -> Option<&'c IdentifierReference<'a>> {
    match expr {
        Expression::Identifier(ident) => Some(ident.as_ref()),
        expr @ match_member_expression!(Expression) => {
            let member = expr.as_member_expression()?;
            get_base_object(member.object())
        }
        _ => None,
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        //
        // r#"useEffect(() => {})"#,
        // r#"useEffect(() => {}, [])"#,
    ];

    let fail = vec![
        r"
        const [top, setTop] = useState();
        const constant = 1;
        const Component = () => {
            const [state, setState] = useState();
            useEffect(() => {
                setState(1);
            }, [state, setTop, constant]);
        }
        ",
    ];

    Tester::new(NoInfiniteRerenderLoop::NAME, pass, fail).test_and_snapshot();
}
