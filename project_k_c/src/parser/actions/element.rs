use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, EXPR_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::expression::{ExpKind, Expr, Literal};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::parser::actions::shared::Shared;
use crate::parser::errors::EXPECT_STRING_EXPR;
use crate::parser::errorss::ActionError;
use crate::parser::locator::LocatorStrategy;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use crate::{pop_expr, types};
use class::ELEMENT;
use class::{ElementAction, Method};
use macros::pop_token;
use manodae::error::ParseError;

pub struct Element {}

impl ElementAction for Element {
    types!();
    // click expr
    #[pop_token(click_token)]
    fn CLICK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let expr = pop_expr!(_tl_stack.pop_expr(), _errors, click_token);
        let span = click_token.span.to(&expr.span);

        let locator_arg = match Shared::get_locator_arg(&expr) {
            Ok(arg) => arg,
            Err(err) => {
                _errors.push_error(&click_token, &expr.span, err.clone());
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
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }

    //enter expression in expression
    #[pop_token(_in_token, enter_token)]
    fn SENDKEYS(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let locator_expr = pop_expr!(_tl_stack.pop_expr(), _errors, enter_token);
        let text_expr = pop_expr!(_tl_stack.pop_expr(), _errors, enter_token);
        let span = enter_token.span.to(&locator_expr.span);

        let locator_arg = match Shared::get_locator_arg(&locator_expr) {
            Ok(arg) => arg,
            Err(err) => {
                _errors.push_error(&enter_token, &locator_expr.span, err.clone());
                return;
            }
        };

        if text_expr.primitive != Primitives::String {
            _errors.push_error(
                &enter_token,
                &text_expr.span,
                EXPECT_STRING_EXPR.to_string(),
            );
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
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }

    //get attribute expression from element expression
    #[pop_token(_element, _from, _attribute, get_token)]
    fn GET_ATTRIBUTE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let locator_expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&get_token, &span, error);
                return;
            }
        };

        let attribute_expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&get_token, &span, error);
                return;
            }
        };

        if Primitives::String != attribute_expr.primitive {
            _errors.push_error(
                &get_token,
                &attribute_expr.span,
                EXPECT_STRING_EXPR.to_string(),
            );
            return;
        }

        if Primitives::String != locator_expr.primitive {
            _errors.push_error(
                &get_token,
                &locator_expr.span,
                EXPECT_STRING_EXPR.to_string(),
            );
            return;
        }

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &locator_expr.kind {
            Args::Locator(LocatorStrategy::parse(locator))
        } else {
            Args::Expr(locator_expr.clone())
        };

        let attribute_arg = if let ExpKind::Lit(Literal::String(attribute)) = attribute_expr.kind {
            Args::String(attribute)
        } else {
            Args::Expr(attribute_expr)
        };

        let span = get_token.span.to(&locator_expr.span);
        let getter = Getter {
            span,
            method: Method::ELEMENT(ELEMENT::GET_ATTRIBUTE),
            arguments: HashMap::from([
                (ATTRIBUTE_ARGKEY, attribute_arg),
                (LOCATOR_ARGKEY, locator_arg),
            ]),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }

    fn IS_DISPLAYED(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
}
