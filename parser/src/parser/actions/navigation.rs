use std::collections::HashMap;

use crate::a_types;
use crate::parser::translator_stack::TLVec;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use ast::action::Action;
use ast::testcase::TestCase;
use ast::teststep::Teststep;
use class::Method;
use class::NavigationAction;
use class::NAVIGATION;
use macros::pop_token;
use manodae::error::ParseError;

pub struct Navigation {}

impl NavigationAction for Navigation {
    a_types!();
    #[pop_token(back_token)]
    fn BACK(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = back_token.span;
        let test_step = Action::new(span, Method::NAVIGATION(NAVIGATION::BACK), HashMap::new());

        _tl_stack.push_step(Teststep::Action(test_step));
    }

    #[pop_token(forward_token)]
    fn FORWARD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = forward_token.span;
        let test_step = Action::new(
            span,
            Method::NAVIGATION(NAVIGATION::FORWARD),
            HashMap::new(),
        );

        _tl_stack.push_step(Teststep::Action(test_step));
    }

    #[pop_token(refresh_token)]
    fn REFRESH(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = refresh_token.span;
        let test_step = Action::new(
            span,
            Method::NAVIGATION(NAVIGATION::REFRESH),
            HashMap::new(),
        );

        _tl_stack.push_step(Teststep::Action(test_step));
    }
}
