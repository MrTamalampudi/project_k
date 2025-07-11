use crate::ast::arguments::Args;
use crate::ast::testcase::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::{Class, Method, WEB_DRIVER};
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use slr_parser::error::ParseError;
use url::Url;

use crate::keywords::TokenType;
use crate::{class::WEB_DRIVER_ACTION, get_input_from_token_stack};

pub struct Driver {}

impl WEB_DRIVER_ACTION for Driver {
    fn NAVIGATE(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
        let url_ = get_input_from_token_stack!(&token_stack.last());

        //sample and it should be imporved
        let parsed_url = match Url::parse(url_) {
            Ok(parsed_url) => {
                if parsed_url.scheme() != "https" {
                    errors.push(ParseError {
                        token: token_stack.last().unwrap().clone(),
                        message: String::from(VALID_URL_SHCEME),
                        productionEnd: false,
                    })
                }
            }
            Err(_) => errors.push(ParseError {
                token: token_stack.last().unwrap().clone().clone(),
                message: String::from(VALID_URL),
                productionEnd: false,
            }),
        };

        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::WEB_DRIVER,
            Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE),
            vec![Args::String(url_.clone())],
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }

    fn CLOSE(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }

    fn FIND_ELEMENT(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }

    fn GET_CURRENT_URL(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }

    fn GET_PAGE_SOURCE(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }

    fn GET_TITLE(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }
    fn GET_WINDOW_HANDLE(
        testcase: &mut crate::ast::testcase::TestCase,
        token_stack: &mut Vec<crate::token::Token>,
        errors: &mut Vec<slr_parser::error::ParseError<crate::token::Token>>,
    ) -> () {
    }
}
