use std::collections::HashMap;

use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::testcase::{TestCase, TestcaseBody};
use crate::ast::teststep::TestStep;
use crate::class::{Class, Method, WEB_DRIVER};
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use crate::token::Token;
use slr_parser::error::ParseError;
use url::Url;

use crate::keywords::TokenType;
use crate::{class::WebDriverAction, get_input_from_token_stack};

pub struct Driver {}

impl WebDriverAction for Driver {
    fn NAVIGATE(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let url_ = get_input_from_token_stack!(&token_stack.last());

        //sample and it should be imporved
        match Url::parse(url_) {
            Ok(parsed_url) => {
                if parsed_url.scheme() != "https" {
                    errors.push(ParseError {
                        token: token_stack.last().unwrap().clone(),
                        message: String::from(VALID_URL_SHCEME),
                        production_end: false,
                    })
                }
            }
            Err(_) => errors.push(ParseError {
                token: token_stack.last().unwrap().clone().clone(),
                message: String::from(VALID_URL),
                production_end: false,
            }),
        };

        let arguments = HashMap::from([(URL_ARGKEY, Args::String(url_.clone()))]);

        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::WEB_DRIVER,
            Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE),
            arguments,
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }

    fn CLOSE(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn FIND_ELEMENT(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_CURRENT_URL(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_PAGE_SOURCE(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_TITLE(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
    fn GET_WINDOW_HANDLE(
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TestcaseBody>,
        errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
}
