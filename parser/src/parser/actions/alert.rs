use std::collections::HashMap;

use ast::{
    expression::{ExpKind, Literal},
    Action,
    ArgKeys::EXPR_ARGKEY,
    Args, Primitives, Teststep,
};
use class::{AlertAction, Method, ALERT, ELEMENT};
use macros::{pop_expr, pop_token};

use crate::{
    a_types,
    parser::{errors::EXPECT_STRING_EXPR, errorss::ActionError, translator_stack::TLVec},
};

pub struct Alert;

impl AlertAction for Alert {
    a_types!();

    #[pop_token(alert, accept)]
    fn ACCEPT(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error>,
    ) {
        let span = accept.1.start..alert.1.end;
        let action = Action {
            span,
            arguments: HashMap::new(),
            method: Method::ALERT(ALERT::ACCEPT),
            next: None,
        };
        _tl_stack.push_step(Teststep::Action(action));
    }

    #[pop_token(alert, dismiss)]
    fn DISMISS(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error>,
    ) {
        let span = dismiss.1.start..alert.1.end;
        let action = Action {
            span,
            arguments: HashMap::new(),
            method: Method::ALERT(ALERT::ACCEPT),
            next: None,
        };
        _tl_stack.push_step(Teststep::Action(action));
    }

    //enter expression in alert
    #[pop_token(alert, _in_token, enter_token)]
    #[pop_expr(text_expr)]
    fn SEND_KEYS(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error>,
    ) {
        let span = enter_token.1.start..alert.1.end;

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
            HashMap::from([(EXPR_ARGKEY, text_arg)]),
        );

        _tl_stack.push_step(Teststep::Action(action));
    }
}
