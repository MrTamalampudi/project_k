use std::collections::HashMap;

use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::expression::{ExpKind, Literal};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::ELEMENT;
use crate::class::{Class, ElementAction, Method};
use crate::get_input_from_token_stack;
use crate::location::{Span, Span_Trait};
use crate::parser::errors::{EXPECT_EXPR, EXPECT_STRING_EXPR};
use crate::parser::errorss::ActionError;
use crate::parser::locator::LocatorStrategy;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use crate::TokenType;
use manodae::error::ParseError;

pub struct Element {}

impl ElementAction for Element {
    fn CLICK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let locator_token = _token_stack.last();
        let locator = LocatorStrategy::parse(get_input_from_token_stack!(locator_token));
        let span = Span {
            start: _token_stack.first().unwrap().get_start_location(),
            end: _token_stack.last().unwrap().get_end_location(),
        };

        let test_step = TestStep::new(
            span,
            Class::ELEMENT,
            Method::ELEMENT(ELEMENT::CLICK),
            HashMap::from([(LOCATOR_ARGKEY, Args::Locator(locator))]),
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        // clear token_stack after every use
        // token stack is particular to production so it should be cleared
        // before any production using
        _token_stack.clear();
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
        let locator_tl = _tl_stack.pop().unwrap();
        let attribute_tl = _tl_stack.pop().unwrap();
        let _element_token = _token_stack.pop().unwrap();
        let _from_token = _token_stack.pop().unwrap();
        let _attribute_token = _token_stack.pop().unwrap();
        let get_token = _token_stack.pop().unwrap();

        let locator_expr = if let TranslatorStack::Expression(locator_expr) = &locator_tl {
            locator_expr
        } else {
            _errors.push_error(&get_token, &locator_tl.get_span(), EXPECT_EXPR.to_string());
            return;
        };

        let attribute_expr = if let TranslatorStack::Expression(attribute_expr) = attribute_tl {
            attribute_expr
        } else {
            _errors.push_error(
                &get_token,
                &attribute_tl.get_span(),
                EXPECT_EXPR.to_string(),
            );
            return;
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

        _tl_stack.push(TranslatorStack::Getter(getter));
    }
}
