use std::collections::HashMap;

use crate::a_types;
use crate::keywords::NTokenType;
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::TLVec;
use crate::parser::translator_stack::TranslatorStack;
use ast::Action;
use ast::ArgKeys::Args;
use ast::ArgKeys::EXPR_ARGKEY;
use ast::TestCase;
use ast::Teststep;
use ast::VarDecl;
use class::CustomAction;
use class::Method;
use class::CUSTOM;
use macros::pop_expr;
use macros::pop_token;
use manodae::error::ParseError;
use span::SpanData;

pub struct Custom {}

impl CustomAction for Custom {
    a_types!();
    //var ident = var_rhs
    //fetch var_rhs from tl_stack last element;
    #[pop_token(_assign, identifier_token)]
    fn VAR_DECLARATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let identifier = match identifier_token.0 {
            NTokenType::IDENTIFIER(ident) => ident,
            _ => return,
        };

        let var_rhs = match _tl_stack.pop_expr() {
            Some(expr) => expr,
            None => {
                return;
            }
        };

        let span = identifier_token.1.start..var_rhs.get_span().end;
        let var_decl = VarDecl::new(identifier.clone(), var_rhs.primitive, var_rhs, span);

        if let Some(variable) = _testcase.variables.get(&identifier) {
            if variable.to_primitive().ne(&var_decl.type_) {
                if let Some(token) = _token_stack.get(2) {
                    _errors.push_error(
                        &token.1,
                        String::from(format!(
                            "{}, expected {} found {}",
                            MISMATCHED_TYPES,
                            variable.to_primitive().to_string(),
                            var_decl.type_.to_string()
                        )),
                    );
                };
                return;
            }
        }

        _tl_stack.push_step(Teststep::VarDecl(var_decl.clone()));
        _testcase.insert_variable(var_decl);
    }

    //assert expr
    #[pop_token(assert_token)]
    #[pop_expr(expr)]
    fn ASSERT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        if !expr.boolean() {
            _errors.push_error(&expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }
        let teststep = Action::new(
            assert_token.1.start..expr.span.end,
            Method::CUSTOM(CUSTOM::ASSERT),
            HashMap::from([(EXPR_ARGKEY, Args::Expr(expr))]),
        );
        _tl_stack.push_step(Teststep::Action(teststep));
    }
}
