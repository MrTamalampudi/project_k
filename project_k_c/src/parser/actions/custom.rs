use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::Args;
use crate::ast::arguments::EXPR_ARGKEY;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::ast::var_decl::VarDecl;
use crate::class::CustomAction;
use crate::class::CUSTOM;
use crate::keywords::TokenType;
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::TLVec;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use manodae::error::ParseError;
use span::Span;
use span::SpanData;

pub struct Custom {}

impl CustomAction for Custom {
    //var ident = var_rhs
    //fetch var_rhs from tl_stack last element;
    fn VAR_DECLARATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        _token_stack.pop(); //pop "assign" token
        let identifier_token = _token_stack.pop().unwrap();
        _token_stack.pop(); //pop "var" token
        let identifier = match identifier_token.get_token_type() {
            TokenType::IDENTIFIER(ident) => ident,
            _ => return,
        };
        _token_stack.pop(); // pop "var" token

        let var_rhs = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&identifier_token, &span, error);
                return;
            }
        };

        let span = identifier_token.span.to(&var_rhs.get_span());
        let var_decl = VarDecl::new(identifier.clone(), var_rhs.primitive, var_rhs, span);

        if let Some(variable) = _testcase.variables.get(&identifier) {
            if variable.to_primitive().ne(&var_decl.type_) {
                if let Some(token) = _token_stack.get(2) {
                    _errors.push(ParseError {
                        token: token.clone(),
                        message: String::from(format!(
                            "{}, expected {} found {}",
                            MISMATCHED_TYPES,
                            variable.to_primitive().to_string(),
                            var_decl.type_.to_string()
                        )),
                        production_end: false,
                    });
                };
                return;
            }
        }

        _tl_stack.push_step(Teststep::VarDecl(var_decl.clone()));
        _testcase.insert_variable(var_decl);
    }

    //assert expr
    fn ASSERT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let assert_token = _token_stack.pop().unwrap();
        let expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&assert_token, &span, error);
                return;
            }
        };

        if !expr.boolean() {
            _errors.push_error(&assert_token, &expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }
        let teststep = Action::new(
            Span {
                start: assert_token.get_start_location(),
                end: expr.span.end,
            },
            crate::class::Method::CUSTOM(CUSTOM::ASSERT),
            HashMap::from([(EXPR_ARGKEY, Args::Expr(expr))]),
        );
        _tl_stack.push_step(Teststep::Action(teststep));
    }
}
