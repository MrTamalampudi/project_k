use std::collections::HashMap;

use crate::ast::testcase::TestCase;
use crate::ast::testcase::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::NavigationAction;
use crate::class::NAVIGATION;
use crate::class::{Class, Method};
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use slr_parser::error::ParseError;

pub struct Navigation {}

impl NavigationAction for Navigation {
    fn BACK(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TranslatorStack>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::BACK),
            HashMap::new(),
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
    fn FORWARD(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TranslatorStack>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::FORWARD),
            HashMap::new(),
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
    fn REFRESH(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TranslatorStack>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::REFRESH),
            HashMap::new(),
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
}
