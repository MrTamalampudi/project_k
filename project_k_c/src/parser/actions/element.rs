use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::expression::{ExpKind, Expr, Literal};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::class::ELEMENT;
use crate::class::{Class, ElementAction, Method};
use crate::parser::errors::EXPECT_STRING_EXPR;
use crate::parser::errorss::ActionError;
use crate::parser::locator::LocatorStrategy;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::pop_expr;
use crate::token::Token;
use manodae::error::ParseError;

pub struct Element {}

impl ElementAction for Element {
    // click expr
    fn CLICK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let click_token = _token_stack.pop().unwrap();
        let expr = pop_expr!(_tl_stack.pop_expr(), _errors, click_token);
        let span = click_token.span.to(&expr.span);

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &expr.kind {
            Args::Locator(LocatorStrategy::parse(locator))
        } else {
            Args::Expr(expr.clone())
        };

        let action = Action::new(
            span,
            Class::ELEMENT,
            Method::ELEMENT(ELEMENT::CLICK),
            HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
        );

        _testcase.insert_teststep(Teststep::Action(action));
    }
    fn CLEAR(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn SENDKEYS(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn SUBMIT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }

    //get attribute expression from element expression
    fn GET_ATTRIBUTE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        _token_stack.pop().unwrap(); // pop "element" token
        _token_stack.pop().unwrap(); // pop "from" token
        _token_stack.pop().unwrap(); // pop "attribute" token
        let get_token = _token_stack.pop().unwrap();

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
}
