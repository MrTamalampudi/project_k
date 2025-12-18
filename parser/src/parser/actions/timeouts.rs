use std::collections::HashMap;

use crate::a_types;
use crate::parser::errors::{EXPECT_NUMBER_EXPR, NEGATIVE_TIME};
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use ast::action::Action;
use ast::arguments::{Args, EXPR_ARGKEY};
use ast::expression::{ExpKind, Literal};
use ast::primitives::Primitives;
use ast::testcase::TestCase;
use ast::teststep::Teststep;
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
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        if Primitives::Number != expr.primitive {
            _errors.push_error(&wait_token, &expr.span, EXPECT_NUMBER_EXPR.to_string());
            return;
        }

        if let ExpKind::Lit(Literal::Number(secs)) = expr.kind {
            if secs < 0 {
                _errors.push_error(&wait_token, &expr.span, NEGATIVE_TIME.to_string());
                return;
            }
        }

        let span = wait_token.span.to(&expr.span);
        let action = Action::new(
            span,
            Method::TIMEOUTS(TIMEOUTS::WAIT),
            HashMap::from([(EXPR_ARGKEY, Args::Expr(expr))]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
}
