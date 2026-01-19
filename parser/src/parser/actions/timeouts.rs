use std::collections::HashMap;

use crate::a_types;
use crate::parser::errors::{EXPECT_NUMBER_EXPR, NEGATIVE_TIME};
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use ast::expression::{ExpKind, Literal};
use ast::Action;
use ast::ArgKeys::{Args, EXPR_ARGKEY};
use ast::Primitives;
use ast::TestCase;
use ast::Teststep;
use class::{Method, TimeoutsAction, TIMEOUTS};
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;

pub struct Timeouts;

impl TimeoutsAction for Timeouts {
    a_types!();
    //action: wait expression
    #[pop_token(wait_token)]
    #[pop_expr(expr)]
    fn WAIT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        if Primitives::Number != expr.primitive {
            _errors.push_error(&expr.span, EXPECT_NUMBER_EXPR.to_string());
            return;
        }

        if let ExpKind::Lit(Literal::Number(secs)) = expr.kind {
            if secs < 0.0 {
                _errors.push_error(&expr.span, NEGATIVE_TIME.to_string());
                return;
            }
        }

        let span = wait_token.1.start..expr.span.end;
        let action = Action::new(
            span,
            Method::TIMEOUTS(TIMEOUTS::WAIT),
            HashMap::from([(EXPR_ARGKEY, Args::Expr(expr))]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
}
