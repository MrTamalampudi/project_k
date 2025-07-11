use slr_parser::error::ParseError;
use slr_parser::grammar;
use slr_parser::grammar::Grammar;
use slr_parser::parser::Parser as SLR_Parser;
use slr_parser::production::Production;
use slr_parser::symbol::Symbol;
use slr_parser::terminal::Terminal;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use url::Url;

use super::Parser;
use crate::ast::arguments::Args;
use crate::ast::if_stmt::IfStmt;
use crate::ast::testcase::{TestCase, TestcaseBody};
use crate::ast::teststep::TestStep;
use crate::ast::AST;
use crate::class::{Class, Method, ELEMENT, NAVIGATION, NAVIGATION_ACTION, WEB_DRIVER};
use crate::class::{ELEMENT_ACTION, WEB_DRIVER_ACTION};
use crate::engine::execute;
use crate::error_handler::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::actions::driver::Driver;
use crate::parser::actions::element::Element;
use crate::parser::actions::navigation::Navigation;
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use crate::parser::locator::LocatorStrategy;
use crate::program::Program;
use crate::token::Token;

#[macro_export]
macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => return,
        }
    };
}

#[macro_export]
macro_rules! get_input_from_token_stack {
    ($input:expr) => {
        match $input {
            Some(value_) => match &value_.token_type {
                TokenType::IDENTIFIER(ident) => ident,
                TokenType::STRING(string) => string,
                _ => return,
            },
            None => return,
        }
    };
}

#[allow(non_camel_case_types, unused)]
pub fn parser_slr(parser: &mut Parser) {
    let tt: Vec<Token> = parser
        .lexer
        .tokens
        .iter()
        .cloned()
        .filter(|t| t.get_token_type().ne(&TokenType::NEW_LINE))
        .collect();
    let d_string = || "".to_string();
    let gr: Grammar<AST, Token> = grammar!(
        TokenType,
        AST,
        Token,
        TESTCASE -> TESTCASE_ TESTSTEPS {error:"Testing"};

        TESTCASE_ -> [TokenType::TESTCASE]
        {action:|ast,token_stack,errors| {
            ast.push(AST::TESTCASE(TestCase::new()));
        }}
        ;

        TESTSTEPS -> TESTSTEPS_ TESTSTEPS_BODY_ {error:"Teststeps body_ 33"};

        TESTSTEPS_ -> [TokenType::TESTSTEPS]
        {action:|ast,token_stack,errors| {
            token_stack.clear();
        }}
        ;

        TESTSTEPS_BODY_ -> TESTSTEPS_BODY
        {error:"Teststeps body_"}
        | TESTSTEPS_BODY TESTSTEPS_BODY_
        {error:"Teststeps body_ 2"};

        TESTSTEPS_BODY -> [ TokenType::ACTION_NAVIGATE,TokenType::STRING(d_string())]
        {error:"Expected syntax ' navigate \"url\" '"}
        {action:|ast,token_stack,errors| {
            Driver::NAVIGATE(ast, token_stack, errors);
        }}
        |
        [TokenType::IDENTIFIER(d_string()),TokenType::ASSIGN_OP] I_S
        {action:|ast,token_stack,errors| {
            let identifier_token = token_stack.first();
            let value_token = token_stack.last();

            if let Some(value) = value_token {

            }

            let name = get_input_from_token_stack!(token_stack.first()) ;
            let value = get_input_from_token_stack!(token_stack.last());

            token_stack.clear();
        }}
        |
        [TokenType::ACTION_CLICK,TokenType::STRING(d_string())]
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,errors| {
            Element::CLICK(ast, token_stack, errors);
        }}
        |
        [TokenType::ACTION_BACK]
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,errors| {
            Navigation::BACK(ast, token_stack, errors);
        }}
        |
        [TokenType::ACTION_FORWARD]
        {action:|ast,token_stack,errors| {
            Navigation::FORWARD(ast, token_stack, errors);
        }}
        |
        [TokenType::ACTION_REFRESH]
        {action:|ast,token_stack,errors| {
            Navigation::REFRESH(ast, token_stack, errors);
        }}
        ;

        I_S -> [TokenType::IDENTIFIER(d_string())] | [TokenType::STRING(d_string())]
    );
    let mut parsed = SLR_Parser::new(gr.productions);
    parsed.compute_lr0_items();
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    let mut ast: Vec<AST> = Vec::new();
    parsed.parse(tt, &mut errors, &mut ast);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    parser.ctx.program = Program {
        testcase: match AST::get_testcase_from_ast(ast.first_mut()) {
            Some(testcase) => testcase.clone(),
            None => return,
        },
    };
    execute(parser.ctx.program.testcase.clone());
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.productionEnd)
        .for_each(|e| e.token.start = e.token.end);
}

fn element_click(
    testcase: &mut TestCase,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
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

fn navigation_back(
    testcase: &mut TestCase,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
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

fn navigation_forward(
    testcase: &mut TestCase,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
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
