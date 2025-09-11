use std::collections::HashMap;

use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::NavigationAction;
use crate::class::NAVIGATION;
use crate::class::{Class, Method};
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use manodae::error::ParseError;

pub struct Navigation {}

impl NavigationAction for Navigation {
    fn BACK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            _token_stack.first().unwrap().get_start_location(),
            _token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::BACK),
            HashMap::new(),
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
    fn FORWARD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            _token_stack.first().unwrap().get_start_location(),
            _token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::FORWARD),
            HashMap::new(),
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
    fn REFRESH(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let test_step = TestStep::new(
            _token_stack.first().unwrap().get_start_location(),
            _token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::REFRESH),
            HashMap::new(),
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
}
