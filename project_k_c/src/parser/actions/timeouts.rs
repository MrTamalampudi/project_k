use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::{Args, EXPR_ARGKEY};
use crate::ast::expression::{ExpKind, Literal};
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::class::{Method, TimeoutsAction, TIMEOUTS};
use crate::parser::errors::{EXPECT_NUMBER_EXPR, NEGATIVE_TIME};
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use manodae::error::ParseError;

pub struct Timeouts;

impl TimeoutsAction for Timeouts {
    //action: wait expression
    fn WAIT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let secs_token = _token_stack.pop().unwrap();
        let expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&secs_token, &span, error);
                return;
            }
        };

        if Primitives::Number != expr.primitive {
            _errors.push_error(&secs_token, &expr.span, EXPECT_NUMBER_EXPR.to_string());
            return;
        }

        if let ExpKind::Lit(Literal::Number(secs)) = expr.kind {
            if secs < 0 {
                _errors.push_error(&secs_token, &expr.span, NEGATIVE_TIME.to_string());
                return;
            }
        }

        let span = secs_token.span.to(&expr.span);
        let action = Action::new(
            span,
            Method::TIMEOUTS(TIMEOUTS::WAIT),
            HashMap::from([(EXPR_ARGKEY, Args::Expr(expr))]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
}
