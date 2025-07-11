use crate::ast::arguments::Args;
use crate::ast::testcase::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::ast::AST;
use crate::class::ELEMENT;
use crate::class::{Class, Method, ELEMENT_ACTION};
use crate::parser::locator::LocatorStrategy;
use crate::token::Token;
use crate::TokenType;
use crate::{get_input_from_token_stack, unwrap_or_return};
use slr_parser::error::ParseError;

pub struct Element {}

impl ELEMENT_ACTION for Element {
    fn CLICK(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let testcase = unwrap_or_return!(AST::get_testcase_from_ast(ast.first_mut()));
        let locator_token = token_stack.last();
        let locator = LocatorStrategy::parse(get_input_from_token_stack!(locator_token));
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::ELEMENT,
            Method::ELEMENT(ELEMENT::CLICK),
            vec![Args::Locator(locator)],
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        // clear token_stack after every use
        // token stack is particular to production so it should be cleared
        // before any production using
        token_stack.clear();
    }
    fn CLEAR(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_ACCESSBILE_NAME(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_ARIA_ROLE(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_ATTRIBUTE(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_CSS_VALUE(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_DOM_PROPERTY(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_LOCATION(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_SIZE(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_TAG_NAME(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn GET_TEXT(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn IS_DISPLAYED(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn IS_ENABLED(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn IS_SELECTED(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn SENDKEYS(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
    fn SUBMIT(
        ast: &mut Vec<AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<slr_parser::error::ParseError<Token>>,
    ) {
    }
}
