use std::collections::HashMap;

use crate::ast::arguments::{Args, LOCATOR_ARGKEY};
use crate::ast::testcase::{TestCase, TestcaseBody};
use crate::ast::teststep::TestStep;
use crate::class::ELEMENT;
use crate::class::{Class, ElementAction, Method};
use crate::get_input_from_token_stack;
use crate::parser::locator::LocatorStrategy;
use crate::token::Token;
use crate::TokenType;
use slr_parser::error::ParseError;

pub struct Element {}

impl ElementAction for Element {
    fn CLICK(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let locator_token = token_stack.last();
        let locator = LocatorStrategy::parse(get_input_from_token_stack!(locator_token));
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::ELEMENT,
            Method::ELEMENT(ELEMENT::CLICK),
            HashMap::from([(LOCATOR_ARGKEY, Args::Locator(locator))]),
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        // clear token_stack after every use
        // token stack is particular to production so it should be cleared
        // before any production using
        token_stack.clear();
    }
    fn CLEAR(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn SENDKEYS(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn SUBMIT(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
    }
}
