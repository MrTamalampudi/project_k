use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::class::Method;
use crate::class::NavigationAction;
use crate::class::NAVIGATION;
use crate::location::Span;
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
        let span = Span {
            start: _token_stack.first().unwrap().get_start_location(),
            end: _token_stack.last().unwrap().get_end_location(),
        };
        let test_step = Action::new(span, Method::NAVIGATION(NAVIGATION::BACK), HashMap::new());

        _testcase.insert_teststep(Teststep::Action(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
    fn FORWARD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = Span {
            start: _token_stack.first().unwrap().get_start_location(),
            end: _token_stack.last().unwrap().get_end_location(),
        };
        let test_step = Action::new(
            span,
            Method::NAVIGATION(NAVIGATION::FORWARD),
            HashMap::new(),
        );

        _testcase.insert_teststep(Teststep::Action(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
    fn REFRESH(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = Span {
            start: _token_stack.first().unwrap().get_start_location(),
            end: _token_stack.last().unwrap().get_end_location(),
        };
        let test_step = Action::new(
            span,
            Method::NAVIGATION(NAVIGATION::REFRESH),
            HashMap::new(),
        );

        _testcase.insert_teststep(Teststep::Action(test_step));

        //clear token_stack after every use
        _token_stack.clear();
    }
}
