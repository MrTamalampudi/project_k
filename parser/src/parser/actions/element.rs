use std::collections::HashMap;

use crate::a_types;
use crate::parser::actions::shared::Shared;
use crate::parser::errors::EXPECT_STRING_EXPR;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use ast::action::Action;
use ast::arguments::{Args, ATTRIBUTE_ARGKEY, EXPR_ARGKEY, LOCATOR_ARGKEY};
use ast::expression::{ExpKind, Expr, Literal};
use ast::getter::Getter;
use ast::locator::LocatorStrategy;
use ast::primitives::Primitives;
use ast::testcase::TestCase;
use ast::teststep::Teststep;
use class::ELEMENT;
use class::{ElementAction, Method};
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;

pub struct Element {}

impl Element {
    fn is_common(
        start_token: Token,
        end_token: Token,
        expr: Expr,
        method: ELEMENT,
        e: &mut Vec<ParseError<Token>>,
        t: &mut Vec<TranslatorStack>,
    ) {
        let span = start_token.span.to(&end_token.span);
        let locator_arg = match Shared::get_locator_arg(&expr) {
            Ok(arg) => arg,
            Err(err) => {
                e.push_error(&start_token, &expr.span, err.clone());
                return;
            }
        };
        let getter = Getter {
            span,
            method: Method::ELEMENT(method),
            arguments: HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::Boolean,
        };
        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::Boolean,
        };
        t.push_expr(expr);
    }
}

impl ElementAction for Element {
    a_types!();
    // click expr
    #[pop_token(click_token)]
    #[pop_expr(expr)]
    fn CLICK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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

    //enter expression in element expression
    #[pop_token(_element_token, _in_token, enter_token)]
    #[pop_expr(locator_expr, text_expr)]
    fn SENDKEYS(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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
    #[pop_expr(locator_expr, attribute_expr)]
    fn GET_ATTRIBUTE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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
            Args::Locator(LocatorStrategy::parse(&locator))
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

    //action: is element expression displayed
    #[pop_token(displayed, _element, is)]
    #[pop_expr(expr)]
    fn IS_DISPLAYED(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        Element::is_common(
            is,
            displayed,
            expr,
            ELEMENT::IS_DISPLAYED,
            _errors,
            _tl_stack,
        );
    }

    #[pop_token(enabled, _element, is)]
    #[pop_expr(expr)]
    fn IS_ENABLED(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        Element::is_common(is, enabled, expr, ELEMENT::IS_ENABLED, _errors, _tl_stack);
    }
    #[pop_token(selected, _element, is)]
    #[pop_expr(expr)]
    fn IS_SELECTED(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        Element::is_common(is, selected, expr, ELEMENT::IS_SELECTED, _errors, _tl_stack);
    }

    fn GET_ACCESSBILE_NAME(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
    }
}
