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
use crate::class::{Class, Element, Method, Navigation, WebDriver};
use crate::engine::execute;
use crate::error_handling::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use crate::parser::locator::LocatorStrategy;
use crate::program::Program;
use crate::token::Token;

#[allow(unused)]
macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => return,
        }
    };
}

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

//This ast is only used as for TranslatorStack
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone)]
enum AST {
    TESTCASE(TestCase),
    TESTSTEP(TestStep),
    IF(IfStmt),
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AST")
    }
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
    let gr: Grammar<TestCase, Token> = grammar!(
        TokenType,
        TestCase,
        Token,
        TESTCASE -> TESTCASE_ TESTSTEPS {error:"Testing"};

        TESTCASE_ -> [TokenType::TESTCASE];

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
            webdriver_navigate(ast, token_stack,errors);
        }}
        |
        [TokenType::IDENTIFIER(d_string()),TokenType::ASSIGN_OP] I_S
        {action:|ast,token_stack,errors| {
            let name = get_input_from_token_stack!(token_stack.first()) ;
            let value = get_input_from_token_stack!(token_stack.last());

            token_stack.clear();
        }}
        |
        [TokenType::ACTION_CLICK,TokenType::STRING(d_string())]
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,errors| {
            element_click(ast, token_stack,errors);
        }}
        |
        [TokenType::ACTION_BACK]
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,errors| {
            navigation_back(ast, token_stack,errors);
        }}
        |
        [TokenType::ACTION_FORWARD]
        {action:|ast,token_stack,errors| {
            navigation_forward(ast, token_stack,errors);
        }}
        ;

        I_S -> [TokenType::IDENTIFIER(d_string())] | [TokenType::STRING(d_string())]
    );
    let mut parsed = SLR_Parser::new(gr.productions);
    parsed.compute_lr0_items();
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    let mut ast = TestCase::new();
    parsed.parse(tt, &mut errors, &mut ast);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    parser.ctx.program = Program {
        testcase: ast.clone(),
    };
    execute(ast);
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.productionEnd)
        .for_each(|e| e.token.start = e.token.end);
}

fn webdriver_navigate(
    testcase: &mut TestCase,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
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
        Method::WEB_DRIVER(WebDriver::NAVIGATE),
        vec![Args::String(url_.clone())],
    );

    testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

    //clear token_stack after every use
    token_stack.clear();
}

fn element_click(
    testcase: &mut TestCase,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
    let locator = LocatorStrategy::parse(get_input_from_token_stack!(token_stack.last()));
    let test_step = TestStep::new(
        token_stack.first().unwrap().get_start_location(),
        token_stack.last().unwrap().get_end_location(),
        Class::ELEMENT,
        Method::ELEMENT(Element::CLICK),
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
        Method::NAVIGATION(Navigation::BACK),
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
        Method::NAVIGATION(Navigation::FORWARD),
        vec![],
    );

    testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));

    //clear token_stack after every use
    token_stack.clear();
}
