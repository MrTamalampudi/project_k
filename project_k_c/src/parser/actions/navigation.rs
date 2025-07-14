use crate::ast::testcase::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::ast::AST;
use crate::class::NavigationAction;
use crate::class::NAVIGATION;
use crate::class::{Class, Method};
use crate::token::Token;
use crate::unwrap_or_return;
use slr_parser::error::ParseError;

pub struct Navigation {}

impl NavigationAction for Navigation {
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
