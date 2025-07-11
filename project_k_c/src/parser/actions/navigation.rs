use crate::ast::arguments::Args;
use crate::ast::testcase::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::ast::AST;
use crate::class::ELEMENT;
use crate::class::NAVIGATION;
use crate::class::NAVIGATION_ACTION;
use crate::class::{Class, Method, ELEMENT_ACTION};
use crate::parser::locator::LocatorStrategy;
use crate::token::Token;
use crate::TokenType;
use crate::{get_input_from_token_stack, unwrap_or_return};
use slr_parser::error::ParseError;

pub struct Navigation {}

impl NAVIGATION_ACTION for Navigation {
    fn BACK(ast: &mut Vec<AST>, token_stack: &mut Vec<Token>, errors: &mut Vec<ParseError<Token>>) {
        let testcase = unwrap_or_return!(AST::get_testcase_from_ast(ast.first_mut()));
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::BACK),
            vec![],
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
    fn FORWARD(
        ast: &mut Vec<crate::ast::AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let testcase = unwrap_or_return!(AST::get_testcase_from_ast(ast.first_mut()));
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::FORWARD),
            vec![],
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
    fn REFRESH(
        ast: &mut Vec<crate::ast::AST>,
        token_stack: &mut Vec<Token>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let testcase = unwrap_or_return!(AST::get_testcase_from_ast(ast.first_mut()));
        let test_step = TestStep::new(
            token_stack.first().unwrap().get_start_location(),
            token_stack.last().unwrap().get_end_location(),
            Class::NAVIGATION,
            Method::NAVIGATION(NAVIGATION::REFRESH),
            vec![],
        );

        testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

        //clear token_stack after every use
        token_stack.clear();
    }
}
