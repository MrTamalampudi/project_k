use std::collections::HashMap;

use crate::ast::arguments::{Args, SECS_ARGKEY};
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::{Method, TimeoutsAction, TIMEOUTS};
use crate::keywords::TokenType;
use crate::location::Span;
use crate::parser::errors::NEGATIVE_TIME;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use manodae::error::ParseError;

pub struct Timeouts {}

impl TimeoutsAction for Timeouts {
    //action: wait 'x'
    fn WAIT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let secs_token = _token_stack.get(1).unwrap();
        if let TokenType::NUMBER(secs) = secs_token.get_token_type() {
            if secs < 0 {
                _errors.push(ParseError {
                    token: secs_token.clone(),
                    message: String::from(NEGATIVE_TIME),
                    production_end: false,
                });
                return;
            } else {
                let span = Span {
                    start: _token_stack.first().unwrap().get_start_location(),
                    end: _token_stack.last().unwrap().get_end_location(),
                };
                let teststep = TestStep::new(
                    span,
                    crate::class::Class::TIMEOUTS,
                    Method::TIMEOUTS(TIMEOUTS::WAIT),
                    HashMap::from([(SECS_ARGKEY, Args::Number(secs))]),
                );
                _testcase.insert_teststep(TestcaseBody::TESTSTEP(teststep));
            }
        }
        _token_stack.clear();
    }
}
