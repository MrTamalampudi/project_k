use std::collections::HashMap;

use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::{Class, Method, WEB_DRIVER};
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use slr_parser::error::ParseError;
use url::Url;

use crate::keywords::TokenType;
use crate::{class::WebDriverAction, get_input_from_token_stack};

pub struct Driver {}

impl WebDriverAction for Driver {
    fn NAVIGATE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let url_ = get_input_from_token_stack!(&_token_stack.last());

        //sample and it should be imporved
        match Url::parse(url_) {
            Ok(parsed_url) => {
                if parsed_url.scheme() != "https" {
                    _errors.push(ParseError {
                        token: _token_stack.last().unwrap().clone(),
                        message: String::from(VALID_URL_SHCEME),
                        production_end: false,
                    })
                }
            }
            Err(_) => _errors.push(ParseError {
                token: _token_stack.last().unwrap().clone().clone(),
                message: String::from(VALID_URL),
                production_end: false,
            }),
        };

        let arguments = HashMap::from([(URL_ARGKEY, Args::String(url_.clone()))]);

        let test_step = TestStep::new(
            _token_stack.first().unwrap().get_start_location(),
            _token_stack.last().unwrap().get_end_location(),
            Class::WEB_DRIVER,
            Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE),
            arguments,
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }

    fn CLOSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let teststep = Getter {
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };

        _tl_stack.push(TranslatorStack::Getter(teststep));
    }

    fn FIND_ELEMENT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_CURRENT_URL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let teststep = Getter {
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };

        _tl_stack.push(TranslatorStack::Getter(teststep));
    }

    fn GET_PAGE_SOURCE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_TITLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
    fn GET_WINDOW_HANDLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
}
