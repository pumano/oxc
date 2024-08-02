#![allow(clippy::wildcard_imports)]
// TODO: I'm not sure if it is a but or intentional but clippy needs this allowed both on this
// module and the generated one.
#![allow(clippy::self_named_module_files)]

//! # Oxc AST
//!
//! Abstract Syntax Tree nodes for Oxc. Supports both TypeScript and JavaScript.
//!
//! This is almost similar to [estree](https://github.com/estree/estree) except a few places:
//! * `Identifier` is replaced with explicit [`BindingIdentifier`], [`IdentifierReference`], [`IdentifierName`] per spec
//! * `AssignmentExpression`.`left` `Pattern` is replaced with [`AssignmentTarget`]
//!
//! ## Parsing
//!
//! You can obtain an AST by parsing source code with a [`Parser`] from [`oxc_parser`].
//!
//! ## Cargo Features
//! * `"serde"` enables support for serde serialization
//!
//! [`BindingIdentifier`]: ast::BindingIdentifier
//! [`IdentifierReference`]: ast::IdentifierReference
//! [`IdentifierName`]: ast::IdentifierName
//! [`AssignmentTarget`]: ast::AssignmentTarget
//! [`oxc_parser`]: <https://docs.rs/oxc_parser>
//! [`Parser`]: <https://docs.rs/oxc_parser/latest/oxc_parser/struct.Parser.html>

#[cfg(feature = "serialize")]
mod serialize;

pub mod ast;
mod ast_builder_impl;
mod ast_impl;
mod ast_kind_impl;
pub mod precedence;
pub mod syntax_directed_operations;
mod trivia;

mod generated {
    pub mod ast_builder;
    pub mod ast_kind;
    pub mod span;
    pub mod visit;
    pub mod visit_mut;
}

pub mod visit {
    pub use crate::generated::visit::*;
    pub use crate::generated::visit_mut::*;
}

pub use generated::ast_builder;
pub use generated::ast_kind;

pub use num_bigint::BigUint;

pub use crate::{
    ast_builder::AstBuilder,
    ast_kind::{AstKind, AstType},
    trivia::{Comment, CommentKind, SortedComments, Trivias},
    visit::{Visit, VisitMut},
};

// After experimenting with two types of boxed enum variants:
//   1.
//   ```
//      enum Expression {
//          Variant(Box<Struct>)
//      }
//      struct Struct {
//          expression: Expression
//      }
//   ```
//   2.
//   ```
//      enum Expression {
//          Variant(Struct)
//      }
//      struct Struct {
//          expression: Box<Expression>
//      }
//   ```
//  I have concluded that the first options is more performant and more ergonomic to use.
//  The following test make sure all enum variants are boxed, resulting 16 bytes for each enum.
//  Read `https://nnethercote.github.io/perf-book/type-sizes.html` for more details.
#[cfg(target_pointer_width = "64")]
#[test]
fn size_asserts() {
    use static_assertions::assert_eq_size;

    use crate::ast;

    assert_eq_size!(ast::Statement, [u8; 16]);
    assert_eq_size!(ast::Expression, [u8; 16]);
    assert_eq_size!(ast::Declaration, [u8; 16]);
    assert_eq_size!(ast::BindingPatternKind, [u8; 16]);
    assert_eq_size!(ast::ModuleDeclaration, [u8; 16]);
    assert_eq_size!(ast::ClassElement, [u8; 16]);
    assert_eq_size!(ast::ExportDefaultDeclarationKind, [u8; 16]);
    assert_eq_size!(ast::AssignmentTargetPattern, [u8; 16]);
    assert_eq_size!(ast::AssignmentTargetMaybeDefault, [u8; 16]);
    assert_eq_size!(ast::AssignmentTargetProperty, [u8; 16]);
    assert_eq_size!(ast::TSLiteral, [u8; 16]);
    assert_eq_size!(ast::TSType, [u8; 16]);
}

#[test]
fn lifetime_variance() {
    use crate::ast;

    fn _assert_program_variant_lifetime<'a: 'b, 'b>(program: ast::Program<'a>) -> ast::Program<'b> {
        program
    }
}

// Assert size of all AST types.
// If AST types are altered (add new fields etc), it's fine to change the values here.
// These assertions are present to ensure that making types `#[repr(C)]` does not alter
// type sizes.
// NB: Must be const assertions, not in a `#[test]` block, in order to test 32-bit type sizes for WASM.
#[cfg(target_pointer_width = "64")]
const _: () = {
    use crate::ast;
    use std::mem::size_of;

    assert!(size_of::<ast::BooleanLiteral>() == 12);
    assert!(size_of::<ast::NullLiteral>() == 8);
    assert!(size_of::<ast::NumericLiteral>() == 40);
    assert!(size_of::<ast::BigIntLiteral>() == 32);
    assert!(size_of::<ast::RegExpLiteral>() == 32);
    assert!(size_of::<ast::RegExp>() == 24);
    assert!(size_of::<ast::EmptyObject>() == 0);
    assert!(size_of::<ast::StringLiteral>() == 24);
    assert!(size_of::<ast::Program>() == 104);
    assert!(size_of::<ast::Expression>() == 16);
    assert!(size_of::<ast::IdentifierName>() == 24);
    assert!(size_of::<ast::IdentifierReference>() == 32);
    assert!(size_of::<ast::BindingIdentifier>() == 32);
    assert!(size_of::<ast::LabelIdentifier>() == 24);
    assert!(size_of::<ast::ThisExpression>() == 8);
    assert!(size_of::<ast::ArrayExpression>() == 56);
    assert!(size_of::<ast::ArrayExpressionElement>() == 16);
    assert!(size_of::<ast::Elision>() == 8);
    assert!(size_of::<ast::ObjectExpression>() == 56);
    assert!(size_of::<ast::ObjectPropertyKind>() == 16);
    assert!(size_of::<ast::ObjectProperty>() == 64);
    assert!(size_of::<ast::PropertyKey>() == 16);
    assert!(size_of::<ast::PropertyKind>() == 1);
    assert!(size_of::<ast::TemplateLiteral>() == 72);
    assert!(size_of::<ast::TaggedTemplateExpression>() == 104);
    assert!(size_of::<ast::TemplateElement>() == 48);
    assert!(size_of::<ast::TemplateElementValue>() == 32);
    assert!(size_of::<ast::MemberExpression>() == 16);
    assert!(size_of::<ast::ComputedMemberExpression>() == 48);
    assert!(size_of::<ast::StaticMemberExpression>() == 56);
    assert!(size_of::<ast::PrivateFieldExpression>() == 56);
    assert!(size_of::<ast::CallExpression>() == 72);
    assert!(size_of::<ast::NewExpression>() == 64);
    assert!(size_of::<ast::MetaProperty>() == 56);
    assert!(size_of::<ast::SpreadElement>() == 24);
    assert!(size_of::<ast::Argument>() == 16);
    assert!(size_of::<ast::UpdateExpression>() == 32);
    assert!(size_of::<ast::UnaryExpression>() == 32);
    assert!(size_of::<ast::BinaryExpression>() == 48);
    assert!(size_of::<ast::PrivateInExpression>() == 56);
    assert!(size_of::<ast::LogicalExpression>() == 48);
    assert!(size_of::<ast::ConditionalExpression>() == 56);
    assert!(size_of::<ast::AssignmentExpression>() == 48);
    assert!(size_of::<ast::AssignmentTarget>() == 16);
    assert!(size_of::<ast::SimpleAssignmentTarget>() == 16);
    assert!(size_of::<ast::AssignmentTargetPattern>() == 16);
    assert!(size_of::<ast::ArrayAssignmentTarget>() == 80);
    assert!(size_of::<ast::ObjectAssignmentTarget>() == 64);
    assert!(size_of::<ast::AssignmentTargetRest>() == 24);
    assert!(size_of::<ast::AssignmentTargetMaybeDefault>() == 16);
    assert!(size_of::<ast::AssignmentTargetWithDefault>() == 40);
    assert!(size_of::<ast::AssignmentTargetProperty>() == 16);
    assert!(size_of::<ast::AssignmentTargetPropertyIdentifier>() == 56);
    assert!(size_of::<ast::AssignmentTargetPropertyProperty>() == 40);
    assert!(size_of::<ast::SequenceExpression>() == 40);
    assert!(size_of::<ast::Super>() == 8);
    assert!(size_of::<ast::AwaitExpression>() == 24);
    assert!(size_of::<ast::ChainExpression>() == 24);
    assert!(size_of::<ast::ChainElement>() == 16);
    assert!(size_of::<ast::ParenthesizedExpression>() == 24);
    assert!(size_of::<ast::Statement>() == 16);
    assert!(size_of::<ast::Directive>() == 48);
    assert!(size_of::<ast::Hashbang>() == 24);
    assert!(size_of::<ast::BlockStatement>() == 48);
    assert!(size_of::<ast::Declaration>() == 16);
    assert!(size_of::<ast::VariableDeclaration>() == 48);
    assert!(size_of::<ast::VariableDeclarationKind>() == 1);
    assert!(size_of::<ast::VariableDeclarator>() == 64);
    assert!(size_of::<ast::UsingDeclaration>() == 48);
    assert!(size_of::<ast::EmptyStatement>() == 8);
    assert!(size_of::<ast::ExpressionStatement>() == 24);
    assert!(size_of::<ast::IfStatement>() == 56);
    assert!(size_of::<ast::DoWhileStatement>() == 40);
    assert!(size_of::<ast::WhileStatement>() == 40);
    assert!(size_of::<ast::ForStatement>() == 80);
    assert!(size_of::<ast::ForStatementInit>() == 16);
    assert!(size_of::<ast::ForInStatement>() == 64);
    assert!(size_of::<ast::ForStatementLeft>() == 16);
    assert!(size_of::<ast::ForOfStatement>() == 64);
    assert!(size_of::<ast::ContinueStatement>() == 32);
    assert!(size_of::<ast::BreakStatement>() == 32);
    assert!(size_of::<ast::ReturnStatement>() == 24);
    assert!(size_of::<ast::WithStatement>() == 40);
    assert!(size_of::<ast::SwitchStatement>() == 64);
    assert!(size_of::<ast::SwitchCase>() == 56);
    assert!(size_of::<ast::LabeledStatement>() == 48);
    assert!(size_of::<ast::ThrowStatement>() == 24);
    assert!(size_of::<ast::TryStatement>() == 32);
    assert!(size_of::<ast::CatchClause>() == 64);
    assert!(size_of::<ast::CatchParameter>() == 40);
    assert!(size_of::<ast::DebuggerStatement>() == 8);
    assert!(size_of::<ast::BindingPattern>() == 32);
    assert!(size_of::<ast::BindingPatternKind>() == 16);
    assert!(size_of::<ast::AssignmentPattern>() == 56);
    assert!(size_of::<ast::ObjectPattern>() == 48);
    assert!(size_of::<ast::BindingProperty>() == 64);
    assert!(size_of::<ast::ArrayPattern>() == 48);
    assert!(size_of::<ast::BindingRestElement>() == 40);
    assert!(size_of::<ast::Function>() == 120);
    assert!(size_of::<ast::FunctionType>() == 1);
    assert!(size_of::<ast::FormalParameters>() == 56);
    assert!(size_of::<ast::FormalParameter>() == 80);
    assert!(size_of::<ast::FormalParameterKind>() == 1);
    assert!(size_of::<ast::FunctionBody>() == 72);
    assert!(size_of::<ast::ArrowFunctionExpression>() == 48);
    assert!(size_of::<ast::YieldExpression>() == 32);
    assert!(size_of::<ast::Class>() == 152);
    assert!(size_of::<ast::ClassType>() == 1);
    assert!(size_of::<ast::ClassBody>() == 40);
    assert!(size_of::<ast::ClassElement>() == 16);
    assert!(size_of::<ast::MethodDefinition>() == 72);
    assert!(size_of::<ast::MethodDefinitionType>() == 1);
    assert!(size_of::<ast::PropertyDefinition>() == 96);
    assert!(size_of::<ast::PropertyDefinitionType>() == 1);
    assert!(size_of::<ast::MethodDefinitionKind>() == 1);
    assert!(size_of::<ast::PrivateIdentifier>() == 24);
    assert!(size_of::<ast::StaticBlock>() == 48);
    assert!(size_of::<ast::ModuleDeclaration>() == 16);
    assert!(size_of::<ast::AccessorPropertyType>() == 1);
    assert!(size_of::<ast::AccessorProperty>() == 80);
    assert!(size_of::<ast::ImportExpression>() == 56);
    assert!(size_of::<ast::ImportDeclaration>() == 136);
    assert!(size_of::<ast::ImportDeclarationSpecifier>() == 16);
    assert!(size_of::<ast::ImportSpecifier>() == 88);
    assert!(size_of::<ast::ImportDefaultSpecifier>() == 40);
    assert!(size_of::<ast::ImportNamespaceSpecifier>() == 40);
    assert!(size_of::<ast::WithClause>() == 64);
    assert!(size_of::<ast::ImportAttribute>() == 64);
    assert!(size_of::<ast::ImportAttributeKey>() == 32);
    assert!(size_of::<ast::ExportNamedDeclaration>() == 152);
    assert!(size_of::<ast::ExportDefaultDeclaration>() == 64);
    assert!(size_of::<ast::ExportAllDeclaration>() == 144);
    assert!(size_of::<ast::ExportSpecifier>() == 96);
    assert!(size_of::<ast::ExportDefaultDeclarationKind>() == 16);
    assert!(size_of::<ast::ModuleExportName>() == 40);
    assert!(size_of::<ast::TSThisParameter>() == 40);
    assert!(size_of::<ast::TSEnumDeclaration>() == 80);
    assert!(size_of::<ast::TSEnumMember>() == 40);
    assert!(size_of::<ast::TSEnumMemberName>() == 16);
    assert!(size_of::<ast::TSTypeAnnotation>() == 24);
    assert!(size_of::<ast::TSLiteralType>() == 24);
    assert!(size_of::<ast::TSLiteral>() == 16);
    assert!(size_of::<ast::TSType>() == 16);
    assert!(size_of::<ast::TSConditionalType>() == 80);
    assert!(size_of::<ast::TSUnionType>() == 40);
    assert!(size_of::<ast::TSIntersectionType>() == 40);
    assert!(size_of::<ast::TSParenthesizedType>() == 24);
    assert!(size_of::<ast::TSTypeOperator>() == 32);
    assert!(size_of::<ast::TSTypeOperatorOperator>() == 1);
    assert!(size_of::<ast::TSArrayType>() == 24);
    assert!(size_of::<ast::TSIndexedAccessType>() == 40);
    assert!(size_of::<ast::TSTupleType>() == 40);
    assert!(size_of::<ast::TSNamedTupleMember>() == 56);
    assert!(size_of::<ast::TSOptionalType>() == 24);
    assert!(size_of::<ast::TSRestType>() == 24);
    assert!(size_of::<ast::TSTupleElement>() == 16);
    assert!(size_of::<ast::TSAnyKeyword>() == 8);
    assert!(size_of::<ast::TSStringKeyword>() == 8);
    assert!(size_of::<ast::TSBooleanKeyword>() == 8);
    assert!(size_of::<ast::TSNumberKeyword>() == 8);
    assert!(size_of::<ast::TSNeverKeyword>() == 8);
    assert!(size_of::<ast::TSIntrinsicKeyword>() == 8);
    assert!(size_of::<ast::TSUnknownKeyword>() == 8);
    assert!(size_of::<ast::TSNullKeyword>() == 8);
    assert!(size_of::<ast::TSUndefinedKeyword>() == 8);
    assert!(size_of::<ast::TSVoidKeyword>() == 8);
    assert!(size_of::<ast::TSSymbolKeyword>() == 8);
    assert!(size_of::<ast::TSThisType>() == 8);
    assert!(size_of::<ast::TSObjectKeyword>() == 8);
    assert!(size_of::<ast::TSBigIntKeyword>() == 8);
    assert!(size_of::<ast::TSTypeReference>() == 32);
    assert!(size_of::<ast::TSTypeName>() == 16);
    assert!(size_of::<ast::TSQualifiedName>() == 48);
    assert!(size_of::<ast::TSTypeParameterInstantiation>() == 40);
    assert!(size_of::<ast::TSTypeParameter>() == 80);
    assert!(size_of::<ast::TSTypeParameterDeclaration>() == 40);
    assert!(size_of::<ast::TSTypeAliasDeclaration>() == 72);
    assert!(size_of::<ast::TSAccessibility>() == 1);
    assert!(size_of::<ast::TSClassImplements>() == 32);
    assert!(size_of::<ast::TSInterfaceDeclaration>() == 96);
    assert!(size_of::<ast::TSInterfaceBody>() == 40);
    assert!(size_of::<ast::TSPropertySignature>() == 40);
    assert!(size_of::<ast::TSSignature>() == 16);
    assert!(size_of::<ast::TSIndexSignature>() == 56);
    assert!(size_of::<ast::TSCallSignatureDeclaration>() == 72);
    assert!(size_of::<ast::TSMethodSignatureKind>() == 1);
    assert!(size_of::<ast::TSMethodSignature>() == 96);
    assert!(size_of::<ast::TSConstructSignatureDeclaration>() == 40);
    assert!(size_of::<ast::TSIndexSignatureName>() == 32);
    assert!(size_of::<ast::TSInterfaceHeritage>() == 32);
    assert!(size_of::<ast::TSTypePredicate>() == 40);
    assert!(size_of::<ast::TSTypePredicateName>() == 16);
    assert!(size_of::<ast::TSModuleDeclaration>() == 64);
    assert!(size_of::<ast::TSModuleDeclarationKind>() == 1);
    assert!(size_of::<ast::TSModuleDeclarationName>() == 32);
    assert!(size_of::<ast::TSModuleDeclarationBody>() == 16);
    assert!(size_of::<ast::TSModuleBlock>() == 72);
    assert!(size_of::<ast::TSTypeLiteral>() == 40);
    assert!(size_of::<ast::TSInferType>() == 16);
    assert!(size_of::<ast::TSTypeQuery>() == 32);
    assert!(size_of::<ast::TSTypeQueryExprName>() == 16);
    assert!(size_of::<ast::TSImportType>() == 96);
    assert!(size_of::<ast::TSImportAttributes>() == 40);
    assert!(size_of::<ast::TSImportAttribute>() == 56);
    assert!(size_of::<ast::TSImportAttributeName>() == 32);
    assert!(size_of::<ast::TSFunctionType>() == 72);
    assert!(size_of::<ast::TSConstructorType>() == 40);
    assert!(size_of::<ast::TSMappedType>() == 56);
    assert!(size_of::<ast::TSMappedTypeModifierOperator>() == 1);
    assert!(size_of::<ast::TSTemplateLiteralType>() == 72);
    assert!(size_of::<ast::TSAsExpression>() == 40);
    assert!(size_of::<ast::TSSatisfiesExpression>() == 40);
    assert!(size_of::<ast::TSTypeAssertion>() == 40);
    assert!(size_of::<ast::TSImportEqualsDeclaration>() == 64);
    assert!(size_of::<ast::TSModuleReference>() == 16);
    assert!(size_of::<ast::TSExternalModuleReference>() == 32);
    assert!(size_of::<ast::TSNonNullExpression>() == 24);
    assert!(size_of::<ast::Decorator>() == 24);
    assert!(size_of::<ast::TSExportAssignment>() == 24);
    assert!(size_of::<ast::TSNamespaceExportDeclaration>() == 32);
    assert!(size_of::<ast::TSInstantiationExpression>() == 32);
    assert!(size_of::<ast::ImportOrExportKind>() == 1);
    assert!(size_of::<ast::JSDocNullableType>() == 32);
    assert!(size_of::<ast::JSDocNonNullableType>() == 32);
    assert!(size_of::<ast::JSDocUnknownType>() == 8);
    assert!(size_of::<ast::JSXElement>() == 56);
    assert!(size_of::<ast::JSXOpeningElement>() == 72);
    assert!(size_of::<ast::JSXClosingElement>() == 24);
    assert!(size_of::<ast::JSXFragment>() == 56);
    assert!(size_of::<ast::JSXOpeningFragment>() == 8);
    assert!(size_of::<ast::JSXClosingFragment>() == 8);
    assert!(size_of::<ast::JSXElementName>() == 16);
    assert!(size_of::<ast::JSXNamespacedName>() == 56);
    assert!(size_of::<ast::JSXMemberExpression>() == 48);
    assert!(size_of::<ast::JSXMemberExpressionObject>() == 16);
    assert!(size_of::<ast::JSXExpressionContainer>() == 24);
    assert!(size_of::<ast::JSXExpression>() == 16);
    assert!(size_of::<ast::JSXEmptyExpression>() == 8);
    assert!(size_of::<ast::JSXAttributeItem>() == 16);
    assert!(size_of::<ast::JSXAttribute>() == 40);
    assert!(size_of::<ast::JSXSpreadAttribute>() == 24);
    assert!(size_of::<ast::JSXAttributeName>() == 16);
    assert!(size_of::<ast::JSXAttributeValue>() == 16);
    assert!(size_of::<ast::JSXIdentifier>() == 24);
    assert!(size_of::<ast::JSXChild>() == 16);
    assert!(size_of::<ast::JSXSpreadChild>() == 24);
    assert!(size_of::<ast::JSXText>() == 24);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    use crate::ast;
    use std::mem::size_of;

    assert!(size_of::<ast::BooleanLiteral>() == 12);
    assert!(size_of::<ast::NullLiteral>() == 8);
    assert!(size_of::<ast::NumericLiteral>() == 32);
    assert!(size_of::<ast::BigIntLiteral>() == 20);
    assert!(size_of::<ast::RegExpLiteral>() == 20);
    assert!(size_of::<ast::RegExp>() == 12);
    assert!(size_of::<ast::EmptyObject>() == 0);
    assert!(size_of::<ast::StringLiteral>() == 16);
    assert!(size_of::<ast::Program>() == 64);
    assert!(size_of::<ast::Expression>() == 8);
    assert!(size_of::<ast::IdentifierName>() == 16);
    assert!(size_of::<ast::IdentifierReference>() == 24);
    assert!(size_of::<ast::BindingIdentifier>() == 20);
    assert!(size_of::<ast::LabelIdentifier>() == 16);
    assert!(size_of::<ast::ThisExpression>() == 8);
    assert!(size_of::<ast::ArrayExpression>() == 36);
    assert!(size_of::<ast::ArrayExpressionElement>() == 12);
    assert!(size_of::<ast::Elision>() == 8);
    assert!(size_of::<ast::ObjectExpression>() == 36);
    assert!(size_of::<ast::ObjectPropertyKind>() == 8);
    assert!(size_of::<ast::ObjectProperty>() == 36);
    assert!(size_of::<ast::PropertyKey>() == 8);
    assert!(size_of::<ast::PropertyKind>() == 1);
    assert!(size_of::<ast::TemplateLiteral>() == 40);
    assert!(size_of::<ast::TaggedTemplateExpression>() == 60);
    assert!(size_of::<ast::TemplateElement>() == 28);
    assert!(size_of::<ast::TemplateElementValue>() == 16);
    assert!(size_of::<ast::MemberExpression>() == 8);
    assert!(size_of::<ast::ComputedMemberExpression>() == 28);
    assert!(size_of::<ast::StaticMemberExpression>() == 36);
    assert!(size_of::<ast::PrivateFieldExpression>() == 36);
    assert!(size_of::<ast::CallExpression>() == 40);
    assert!(size_of::<ast::NewExpression>() == 36);
    assert!(size_of::<ast::MetaProperty>() == 40);
    assert!(size_of::<ast::SpreadElement>() == 16);
    assert!(size_of::<ast::Argument>() == 8);
    assert!(size_of::<ast::UpdateExpression>() == 20);
    assert!(size_of::<ast::UnaryExpression>() == 20);
    assert!(size_of::<ast::BinaryExpression>() == 28);
    assert!(size_of::<ast::PrivateInExpression>() == 36);
    assert!(size_of::<ast::LogicalExpression>() == 28);
    assert!(size_of::<ast::ConditionalExpression>() == 32);
    assert!(size_of::<ast::AssignmentExpression>() == 28);
    assert!(size_of::<ast::AssignmentTarget>() == 8);
    assert!(size_of::<ast::SimpleAssignmentTarget>() == 8);
    assert!(size_of::<ast::AssignmentTargetPattern>() == 8);
    assert!(size_of::<ast::ArrayAssignmentTarget>() == 52);
    assert!(size_of::<ast::ObjectAssignmentTarget>() == 40);
    assert!(size_of::<ast::AssignmentTargetRest>() == 16);
    assert!(size_of::<ast::AssignmentTargetMaybeDefault>() == 8);
    assert!(size_of::<ast::AssignmentTargetWithDefault>() == 24);
    assert!(size_of::<ast::AssignmentTargetProperty>() == 8);
    assert!(size_of::<ast::AssignmentTargetPropertyIdentifier>() == 40);
    assert!(size_of::<ast::AssignmentTargetPropertyProperty>() == 24);
    assert!(size_of::<ast::SequenceExpression>() == 24);
    assert!(size_of::<ast::Super>() == 8);
    assert!(size_of::<ast::AwaitExpression>() == 16);
    assert!(size_of::<ast::ChainExpression>() == 16);
    assert!(size_of::<ast::ChainElement>() == 8);
    assert!(size_of::<ast::ParenthesizedExpression>() == 16);
    assert!(size_of::<ast::Statement>() == 8);
    assert!(size_of::<ast::Directive>() == 32);
    assert!(size_of::<ast::Hashbang>() == 16);
    assert!(size_of::<ast::BlockStatement>() == 28);
    assert!(size_of::<ast::Declaration>() == 8);
    assert!(size_of::<ast::VariableDeclaration>() == 28);
    assert!(size_of::<ast::VariableDeclarationKind>() == 1);
    assert!(size_of::<ast::VariableDeclarator>() == 36);
    assert!(size_of::<ast::UsingDeclaration>() == 28);
    assert!(size_of::<ast::EmptyStatement>() == 8);
    assert!(size_of::<ast::ExpressionStatement>() == 16);
    assert!(size_of::<ast::IfStatement>() == 32);
    assert!(size_of::<ast::DoWhileStatement>() == 24);
    assert!(size_of::<ast::WhileStatement>() == 24);
    assert!(size_of::<ast::ForStatement>() == 44);
    assert!(size_of::<ast::ForStatementInit>() == 8);
    assert!(size_of::<ast::ForInStatement>() == 36);
    assert!(size_of::<ast::ForStatementLeft>() == 8);
    assert!(size_of::<ast::ForOfStatement>() == 40);
    assert!(size_of::<ast::ContinueStatement>() == 24);
    assert!(size_of::<ast::BreakStatement>() == 24);
    assert!(size_of::<ast::ReturnStatement>() == 16);
    assert!(size_of::<ast::WithStatement>() == 24);
    assert!(size_of::<ast::SwitchStatement>() == 36);
    assert!(size_of::<ast::SwitchCase>() == 32);
    assert!(size_of::<ast::LabeledStatement>() == 32);
    assert!(size_of::<ast::ThrowStatement>() == 16);
    assert!(size_of::<ast::TryStatement>() == 20);
    assert!(size_of::<ast::CatchClause>() == 40);
    assert!(size_of::<ast::CatchParameter>() == 24);
    assert!(size_of::<ast::DebuggerStatement>() == 8);
    assert!(size_of::<ast::BindingPattern>() == 16);
    assert!(size_of::<ast::BindingPatternKind>() == 8);
    assert!(size_of::<ast::AssignmentPattern>() == 32);
    assert!(size_of::<ast::ObjectPattern>() == 28);
    assert!(size_of::<ast::BindingProperty>() == 36);
    assert!(size_of::<ast::ArrayPattern>() == 28);
    assert!(size_of::<ast::BindingRestElement>() == 24);
    assert!(size_of::<ast::Function>() == 80);
    assert!(size_of::<ast::FunctionType>() == 1);
    assert!(size_of::<ast::FormalParameters>() == 32);
    assert!(size_of::<ast::FormalParameter>() == 44);
    assert!(size_of::<ast::FormalParameterKind>() == 1);
    assert!(size_of::<ast::FunctionBody>() == 40);
    assert!(size_of::<ast::ArrowFunctionExpression>() == 32);
    assert!(size_of::<ast::YieldExpression>() == 20);
    assert!(size_of::<ast::Class>() == 88);
    assert!(size_of::<ast::ClassType>() == 1);
    assert!(size_of::<ast::ClassBody>() == 24);
    assert!(size_of::<ast::ClassElement>() == 8);
    assert!(size_of::<ast::MethodDefinition>() == 44);
    assert!(size_of::<ast::MethodDefinitionType>() == 1);
    assert!(size_of::<ast::PropertyDefinition>() == 56);
    assert!(size_of::<ast::PropertyDefinitionType>() == 1);
    assert!(size_of::<ast::MethodDefinitionKind>() == 1);
    assert!(size_of::<ast::PrivateIdentifier>() == 16);
    assert!(size_of::<ast::StaticBlock>() == 28);
    assert!(size_of::<ast::ModuleDeclaration>() == 8);
    assert!(size_of::<ast::AccessorPropertyType>() == 1);
    assert!(size_of::<ast::AccessorProperty>() == 44);
    assert!(size_of::<ast::ImportExpression>() == 32);
    assert!(size_of::<ast::ImportDeclaration>() == 84);
    assert!(size_of::<ast::ImportDeclarationSpecifier>() == 8);
    assert!(size_of::<ast::ImportSpecifier>() == 60);
    assert!(size_of::<ast::ImportDefaultSpecifier>() == 28);
    assert!(size_of::<ast::ImportNamespaceSpecifier>() == 28);
    assert!(size_of::<ast::WithClause>() == 40);
    assert!(size_of::<ast::ImportAttribute>() == 44);
    assert!(size_of::<ast::ImportAttributeKey>() == 20);
    assert!(size_of::<ast::ExportNamedDeclaration>() == 92);
    assert!(size_of::<ast::ExportDefaultDeclaration>() == 44);
    assert!(size_of::<ast::ExportAllDeclaration>() == 96);
    assert!(size_of::<ast::ExportSpecifier>() == 68);
    assert!(size_of::<ast::ExportDefaultDeclarationKind>() == 8);
    assert!(size_of::<ast::ModuleExportName>() == 28);
    assert!(size_of::<ast::TSThisParameter>() == 28);
    assert!(size_of::<ast::TSEnumDeclaration>() == 52);
    assert!(size_of::<ast::TSEnumMember>() == 24);
    assert!(size_of::<ast::TSEnumMemberName>() == 8);
    assert!(size_of::<ast::TSTypeAnnotation>() == 16);
    assert!(size_of::<ast::TSLiteralType>() == 16);
    assert!(size_of::<ast::TSLiteral>() == 8);
    assert!(size_of::<ast::TSType>() == 8);
    assert!(size_of::<ast::TSConditionalType>() == 44);
    assert!(size_of::<ast::TSUnionType>() == 24);
    assert!(size_of::<ast::TSIntersectionType>() == 24);
    assert!(size_of::<ast::TSParenthesizedType>() == 16);
    assert!(size_of::<ast::TSTypeOperator>() == 20);
    assert!(size_of::<ast::TSTypeOperatorOperator>() == 1);
    assert!(size_of::<ast::TSArrayType>() == 16);
    assert!(size_of::<ast::TSIndexedAccessType>() == 24);
    assert!(size_of::<ast::TSTupleType>() == 24);
    assert!(size_of::<ast::TSNamedTupleMember>() == 36);
    assert!(size_of::<ast::TSOptionalType>() == 16);
    assert!(size_of::<ast::TSRestType>() == 16);
    assert!(size_of::<ast::TSTupleElement>() == 8);
    assert!(size_of::<ast::TSAnyKeyword>() == 8);
    assert!(size_of::<ast::TSStringKeyword>() == 8);
    assert!(size_of::<ast::TSBooleanKeyword>() == 8);
    assert!(size_of::<ast::TSNumberKeyword>() == 8);
    assert!(size_of::<ast::TSNeverKeyword>() == 8);
    assert!(size_of::<ast::TSIntrinsicKeyword>() == 8);
    assert!(size_of::<ast::TSUnknownKeyword>() == 8);
    assert!(size_of::<ast::TSNullKeyword>() == 8);
    assert!(size_of::<ast::TSUndefinedKeyword>() == 8);
    assert!(size_of::<ast::TSVoidKeyword>() == 8);
    assert!(size_of::<ast::TSSymbolKeyword>() == 8);
    assert!(size_of::<ast::TSThisType>() == 8);
    assert!(size_of::<ast::TSObjectKeyword>() == 8);
    assert!(size_of::<ast::TSBigIntKeyword>() == 8);
    assert!(size_of::<ast::TSTypeReference>() == 20);
    assert!(size_of::<ast::TSTypeName>() == 8);
    assert!(size_of::<ast::TSQualifiedName>() == 32);
    assert!(size_of::<ast::TSTypeParameterInstantiation>() == 24);
    assert!(size_of::<ast::TSTypeParameter>() == 48);
    assert!(size_of::<ast::TSTypeParameterDeclaration>() == 24);
    assert!(size_of::<ast::TSTypeAliasDeclaration>() == 48);
    assert!(size_of::<ast::TSAccessibility>() == 1);
    assert!(size_of::<ast::TSClassImplements>() == 20);
    assert!(size_of::<ast::TSInterfaceDeclaration>() == 60);
    assert!(size_of::<ast::TSInterfaceBody>() == 24);
    assert!(size_of::<ast::TSPropertySignature>() == 24);
    assert!(size_of::<ast::TSSignature>() == 8);
    assert!(size_of::<ast::TSIndexSignature>() == 32);
    assert!(size_of::<ast::TSCallSignatureDeclaration>() == 48);
    assert!(size_of::<ast::TSMethodSignatureKind>() == 1);
    assert!(size_of::<ast::TSMethodSignature>() == 64);
    assert!(size_of::<ast::TSConstructSignatureDeclaration>() == 24);
    assert!(size_of::<ast::TSIndexSignatureName>() == 20);
    assert!(size_of::<ast::TSInterfaceHeritage>() == 20);
    assert!(size_of::<ast::TSTypePredicate>() == 28);
    assert!(size_of::<ast::TSTypePredicateName>() == 12);
    assert!(size_of::<ast::TSModuleDeclaration>() == 44);
    assert!(size_of::<ast::TSModuleDeclarationKind>() == 1);
    assert!(size_of::<ast::TSModuleDeclarationName>() == 20);
    assert!(size_of::<ast::TSModuleDeclarationBody>() == 8);
    assert!(size_of::<ast::TSModuleBlock>() == 40);
    assert!(size_of::<ast::TSTypeLiteral>() == 24);
    assert!(size_of::<ast::TSInferType>() == 12);
    assert!(size_of::<ast::TSTypeQuery>() == 20);
    assert!(size_of::<ast::TSTypeQueryExprName>() == 8);
    assert!(size_of::<ast::TSImportType>() == 56);
    assert!(size_of::<ast::TSImportAttributes>() == 24);
    assert!(size_of::<ast::TSImportAttribute>() == 36);
    assert!(size_of::<ast::TSImportAttributeName>() == 20);
    assert!(size_of::<ast::TSFunctionType>() == 48);
    assert!(size_of::<ast::TSConstructorType>() == 24);
    assert!(size_of::<ast::TSMappedType>() == 36);
    assert!(size_of::<ast::TSMappedTypeModifierOperator>() == 1);
    assert!(size_of::<ast::TSTemplateLiteralType>() == 40);
    assert!(size_of::<ast::TSAsExpression>() == 24);
    assert!(size_of::<ast::TSSatisfiesExpression>() == 24);
    assert!(size_of::<ast::TSTypeAssertion>() == 24);
    assert!(size_of::<ast::TSImportEqualsDeclaration>() == 40);
    assert!(size_of::<ast::TSModuleReference>() == 8);
    assert!(size_of::<ast::TSExternalModuleReference>() == 24);
    assert!(size_of::<ast::TSNonNullExpression>() == 16);
    assert!(size_of::<ast::Decorator>() == 16);
    assert!(size_of::<ast::TSExportAssignment>() == 16);
    assert!(size_of::<ast::TSNamespaceExportDeclaration>() == 24);
    assert!(size_of::<ast::TSInstantiationExpression>() == 20);
    assert!(size_of::<ast::ImportOrExportKind>() == 1);
    assert!(size_of::<ast::JSDocNullableType>() == 20);
    assert!(size_of::<ast::JSDocNonNullableType>() == 20);
    assert!(size_of::<ast::JSDocUnknownType>() == 8);
    assert!(size_of::<ast::JSXElement>() == 32);
    assert!(size_of::<ast::JSXOpeningElement>() == 40);
    assert!(size_of::<ast::JSXClosingElement>() == 16);
    assert!(size_of::<ast::JSXFragment>() == 40);
    assert!(size_of::<ast::JSXOpeningFragment>() == 8);
    assert!(size_of::<ast::JSXClosingFragment>() == 8);
    assert!(size_of::<ast::JSXElementName>() == 8);
    assert!(size_of::<ast::JSXNamespacedName>() == 40);
    assert!(size_of::<ast::JSXMemberExpression>() == 32);
    assert!(size_of::<ast::JSXMemberExpressionObject>() == 8);
    assert!(size_of::<ast::JSXExpressionContainer>() == 20);
    assert!(size_of::<ast::JSXExpression>() == 12);
    assert!(size_of::<ast::JSXEmptyExpression>() == 8);
    assert!(size_of::<ast::JSXAttributeItem>() == 8);
    assert!(size_of::<ast::JSXAttribute>() == 24);
    assert!(size_of::<ast::JSXSpreadAttribute>() == 16);
    assert!(size_of::<ast::JSXAttributeName>() == 8);
    assert!(size_of::<ast::JSXAttributeValue>() == 8);
    assert!(size_of::<ast::JSXIdentifier>() == 16);
    assert!(size_of::<ast::JSXChild>() == 8);
    assert!(size_of::<ast::JSXSpreadChild>() == 16);
    assert!(size_of::<ast::JSXText>() == 16);
};
