//! RegExp Transformer
//!
//! This module supports various RegExp plugins to handle unsupported RegExp literal features.
//! When an unsupported feature is detected, these plugins convert the RegExp literal into
//! a `new RegExp()` constructor call to avoid syntax errors.
//!
//! Note: You will need to include a polyfill for the `RegExp` constructor in your code to have the correct runtime behavior.
//!
//! ### ES2015
//!
//! #### Sticky flag (`y`)
//! - @babel/plugin-transform-sticky-regex: <https://babeljs.io/docs/en/babel-plugin-transform-sticky-regex>
//!
//! #### Unicode flag (`u`)
//! - @babel/plugin-transform-unicode-regex: <https://babeljs.io/docs/en/babel-plugin-transform-unicode-regex>
//!
//! ### ES2018
//!
//! #### DotAll flag (`s`)
//! - @babel/plugin-transform-dotall-regex: <https://babeljs.io/docs/en/babel-plugin-transform-dotall-regex>
//! - Spec: ECMAScript 2018: <https://262.ecma-international.org/9.0/#sec-get-regexp.prototype.dotAll>
//!
//! #### Lookbehind assertions (`/(?<=x)/` and `/(?<!x)/`)
//! - Implementation: Same as esbuild's handling
//!
//! #### Named capture groups (`(?<name>x)`)
//! - @babel/plugin-transform-named-capturing-groups-regex: <https://babeljs.io/docs/en/babel-plugin-transform-named-capturing-groups-regex>
//!
//! #### Unicode property escapes (`\p{...}` and `\P{...}`)
//! - @babel/plugin-transform-unicode-property-regex: <https://babeljs.io/docs/en/babel-plugin-proposal-unicode-property-regex>
//!
//! ### ES2022
//!
//! #### Match indices flag (`d`)
//! - Implementation: Same as esbuild's handling
//!
//! ### ES2024
//!
//! #### Set notation + properties of strings (`v`)
//! - @babel/plugin-transform-unicode-sets-regex: <https://babeljs.io/docs/en/babel-plugin-proposal-unicode-sets-regex>
//! - TC39 Proposal: <https://github.com/tc39/proposal-regexp-set-notation>

mod options;

use std::borrow::Cow;

pub use options::RegExpOptions;
use oxc_ast::ast::*;
use oxc_regular_expression::ast::{LookAroundAssertionKind, Pattern, Term};
use oxc_semantic::ReferenceFlags;
use oxc_span::Atom;
use oxc_traverse::Traverse;

use crate::context::Ctx;

pub struct RegExp<'a> {
    ctx: Ctx<'a>,
    options: RegExpOptions,
}

impl<'a> RegExp<'a> {
    pub fn new(options: RegExpOptions, ctx: Ctx<'a>) -> Self {
        Self { ctx, options }
    }
}

impl<'a> Traverse<'a> for RegExp<'a> {
    fn enter_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut oxc_traverse::TraverseCtx<'a>,
    ) {
        let Expression::RegExpLiteral(regexp) = expr else {
            return;
        };

        let is_unsupported = self.has_unsupported_regular_expression_flags(regexp.regex.flags);

        let mut pattern_source = Cow::Borrowed(match regexp.regex.pattern {
            RegExpPattern::Raw(pattern)
                if is_unsupported
                    || self.has_unsupported_regex_syntax_raw(pattern, regexp.regex.flags) =>
            {
                pattern
            }
            RegExpPattern::Pattern(ref pattern)
                if is_unsupported || self.has_unsupported_regular_expression_pattern(pattern) =>
            {
                regexp.regex.pattern.source_text(self.ctx.source_text)
            }
            _ => return,
        });

        if pattern_source.contains('\\') {
            // Escape backslashes in the pattern source
            pattern_source = Cow::Owned(pattern_source.replace('\\', "\\\\"));
        }

        let callee = {
            let symbol_id = ctx.scopes().find_binding(ctx.current_scope_id(), "RegExp");
            let ident = ctx.create_reference_id(
                regexp.span,
                Atom::from("RegExp"),
                symbol_id,
                ReferenceFlags::read(),
            );
            ctx.ast.expression_from_identifier_reference(ident)
        };

        let mut arguments = ctx.ast.vec_with_capacity(2);
        arguments.push(
            ctx.ast.argument_expression(
                ctx.ast.expression_string_literal(regexp.span, pattern_source),
            ),
        );

        let flags = regexp.regex.flags.to_string();
        let flags =
            ctx.ast.argument_expression(ctx.ast.expression_string_literal(regexp.span, flags));
        arguments.push(flags);

        *expr = ctx.ast.expression_new(
            regexp.span,
            callee,
            arguments,
            None::<TSTypeParameterInstantiation>,
        );
    }
}

impl<'a> RegExp<'a> {
    /// Check if the regular expression contains any unsupported flags.
    fn has_unsupported_regular_expression_flags(&mut self, flags: RegExpFlags) -> bool {
        flags.iter().any(|f| match f {
            RegExpFlags::G | RegExpFlags::I | RegExpFlags::M => true,
            RegExpFlags::S if self.options.dot_all_flag => true,
            RegExpFlags::Y if self.options.sticky_flag => true,
            RegExpFlags::U if self.options.unicode_flag => true,
            RegExpFlags::D if self.options.match_indices => true,
            RegExpFlags::V if self.options.set_notation => true,
            _ => false,
        })
    }

    /// Check if the regular expression contains any unsupported syntax.
    ///
    /// Based on parsed regular expression pattern.
    fn has_unsupported_regular_expression_pattern(&mut self, pattern: &Pattern<'a>) -> bool {
        // Early return if no unsupported features-related plugins are enabled
        if !(self.options.named_capture_groups
            || self.options.unicode_property_escapes
            || self.options.look_behind_assertions)
        {
            return false;
        }

        pattern.body.body.iter().any(|alternative| {
            alternative.body.iter().any(|element| match element {
                Term::CapturingGroup(_) if self.options.named_capture_groups => true,
                Term::UnicodePropertyEscape(_) if self.options.unicode_property_escapes => true,
                Term::LookAroundAssertion(assertion)
                    if self.options.look_behind_assertions
                        && matches!(
                            assertion.kind,
                            LookAroundAssertionKind::Lookbehind
                                | LookAroundAssertionKind::NegativeLookbehind
                        ) =>
                {
                    true
                }
                _ => false,
            })
        })
    }

    /// Check if the regular expression contains any unsupported syntax.
    ///
    /// Port from [esbuild](https://github.com/evanw/esbuild/blob/332727499e62315cff4ecaff9fa8b86336555e46/internal/js_parser/js_parser.go#L12667-L12800)
    fn has_unsupported_regex_syntax_raw(&mut self, pattern: &str, flags: RegExpFlags) -> bool {
        let is_unicode = flags.contains(RegExpFlags::U);
        let mut paren_depth = 0;
        let mut chars = pattern.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    while let Some(c) = chars.next() {
                        if c == ']' {
                            break;
                        }
                        if c == '\\' {
                            chars.next();
                        }
                    }
                }
                '(' => {
                    if matches!(chars.peek(), Some(&'?')) {
                        chars.next();
                        if matches!(chars.peek(), Some(&'<')) {
                            chars.next();
                            match chars.peek() {
                                Some(&'!' | &'=') if self.options.look_behind_assertions => {
                                    // (?<=) and (?<!)
                                    return true;
                                }
                                _ => {
                                    if self.options.named_capture_groups {
                                        return chars.any(|c| c == '>');
                                    }
                                }
                            }
                        }
                    }
                    paren_depth += 1;
                }
                ')' => {
                    if paren_depth == 0 {
                        return true;
                    }
                    paren_depth -= 1;
                }
                '\\' => {
                    if self.options.unicode_property_escapes && is_unicode {
                        if let Some(&next_char) = chars.peek() {
                            // \p{ and \P{
                            if (next_char == 'p' || next_char == 'P') && chars.nth(1) == Some('{') {
                                return chars.any(|c| c == '}');
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }
}