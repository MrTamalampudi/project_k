use std::collections::HashMap;

use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::ELEMENT;
use crate::class::{Class, ElementAction, Method};
use crate::get_input_from_token_stack;
use crate::parser::errors::VARIABLE_NOT_DEFINED;
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
        let test_step = TestStep::new(
            _token_stack.first().unwrap().get_start_location(),
            _token_stack.last().unwrap().get_end_location(),
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
    //get attribute ident_or_string from element ident_or_string
    fn GET_ATTRIBUTE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let locator_token = _token_stack
            .get(_token_stack.len().saturating_sub(1))
            .unwrap();
        let attribute_token = _token_stack
            .get(_token_stack.len().saturating_sub(4))
            .unwrap();

        let locator_arg = match &locator_token.token_type {
            TokenType::STRING(locator) => Args::Locator(LocatorStrategy::parse(locator)),
            TokenType::IDENTIFIER(ident) => {
                if let None = _testcase.variables.get(ident) {
                    _errors.push(ParseError {
                        token: locator_token.clone(),
                        message: String::from(VARIABLE_NOT_DEFINED),
                        production_end: false,
                    });
                    Args::None
                } else {
                    Args::Ident(ident.clone())
                }
            }
            _ => {
                _errors.push(ParseError {
                    token: locator_token.clone(),
                    message: String::from(VARIABLE_NOT_DEFINED),
                    production_end: false,
                });
                Args::None
            }
        };

        let attribute_arg = match &attribute_token.token_type {
            TokenType::STRING(attribute) => Args::String(attribute.clone()),
            TokenType::IDENTIFIER(ident) => {
                if let None = _testcase.variables.get(ident) {
                    _errors.push(ParseError {
                        token: locator_token.clone(),
                        message: String::from(VARIABLE_NOT_DEFINED),
                        production_end: false,
                    });
                    Args::None
                } else {
                    Args::Ident(ident.clone())
                }
            }
            _ => {
                _errors.push(ParseError {
                    token: locator_token.clone(),
                    message: String::from(VARIABLE_NOT_DEFINED),
                    production_end: false,
                });
                Args::None
            }
        };

        let teststep = Getter {
            method: Method::ELEMENT(ELEMENT::GET_ATTRIBUTE),
            arguments: HashMap::from([
                (ATTRIBUTE_ARGKEY, attribute_arg),
                (LOCATOR_ARGKEY, locator_arg),
            ]),
            returns: Primitives::String,
        };

        _tl_stack.push(TranslatorStack::Getter(teststep));
    }
}
