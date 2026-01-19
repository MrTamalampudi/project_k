use std::collections::HashMap;

use crate::a_types;
use crate::parser::actions::shared::Shared;
use crate::parser::errors::EXPECT_STRING_EXPR;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use ast::expression::{ExpKind, Literal};
use ast::Action;
use ast::ArgKeys::{Args, EXPR_ARGKEY, LOCATOR_ARGKEY};
use ast::Primitives;
use ast::TestCase;
use ast::Teststep;
use class::ELEMENT;
use class::{ElementAction, Method};
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;

pub struct Element {}

impl ElementAction for Element {
    a_types!();
    // click expr
    #[pop_token(click_token)]
    #[pop_expr(expr)]
    fn CLICK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = click_token.1.start..expr.span.end;

        let locator_arg = match Shared::get_locator_arg(&expr) {
            Ok(arg) => arg,
            Err(err) => {
                _errors.push_error(&expr.span, err.clone());
                return;
            }
        };

        let action = Action::new(
            span,
            Method::ELEMENT(ELEMENT::CLICK),
            HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
    fn CLEAR(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
    }

    //enter expression in element expression
    #[pop_token(_element_token, _in_token, enter_token)]
    #[pop_expr(locator_expr, text_expr)]
    fn SENDKEYS(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = enter_token.1.start..locator_expr.span.end;

        let locator_arg = match Shared::get_locator_arg(&locator_expr) {
            Ok(arg) => arg,
            Err(err) => {
                _errors.push_error(&locator_expr.span, err.clone());
                return;
            }
        };

        if text_expr.primitive != Primitives::String {
            _errors.push_error(&text_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        let text_arg = if let ExpKind::Lit(Literal::String(attribute)) = text_expr.kind {
            Args::String(attribute)
        } else {
            Args::Expr(text_expr)
        };

        let action = Action::new(
            span,
            Method::ELEMENT(ELEMENT::SENDKEYS),
            HashMap::from([(LOCATOR_ARGKEY, locator_arg), (EXPR_ARGKEY, text_arg)]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
    fn SUBMIT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
    }
}
