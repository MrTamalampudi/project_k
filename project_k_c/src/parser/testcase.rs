use slr_parser::error::ParseError;
use slr_parser::grammar;
use slr_parser::grammar::Grammar;
use slr_parser::parser::Parser as SLR_Parser;
use slr_parser::production::Production;
use slr_parser::symbol::Symbol;
use slr_parser::terminal::Terminal;
use std::sync::Arc;
use url::Url;

use super::Parser;
use crate::ast::arguments::Args;
use crate::ast::if_stmt::IfStmt;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::TestStep;
use crate::class::{Class, Element, Method, Navigation, WebDriver};
use crate::engine::execute;
use crate::error_handling::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::errors::{VALID_URL, VALID_URL_SHCEME};
use crate::parser::locator::LocatorStrategy;
use crate::token::Token;

macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr {
            Some(value_) => value_,
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
    let gr: Grammar<AST, Token> = grammar!(
        TokenType,
        AST,
        Token,
        TESTCASE -> TESTCASE_ TESTSTEPS {error:"Testing"};

        TESTCASE_ -> [TokenType::TESTCASE]
        { action: |tl_stack,token_stack,errors| {
            tl_stack.push(AST::TESTCASE(TestCase::new()));
        }};

        TESTSTEPS -> TESTSTEPS_ TESTSTEPS_BODY_ {error:"Teststeps body_ 33"};

        TESTSTEPS_ -> [TokenType::TESTSTEPS]
        {action:|tl_stack,token_stack,errors| {
            token_stack.clear();
        }}
        ;

        TESTSTEPS_BODY_ -> TESTSTEPS_BODY
        {error:"Teststeps body_"}
        | TESTSTEPS_BODY TESTSTEPS_BODY_
        {error:"Teststeps body_ 2"};

        TESTSTEPS_BODY -> [ TokenType::ACTION_NAVIGATE,TokenType::STRING(d_string())]
        {error:"Expected syntax ' navigate \"url\" '"}
        {action:|tl_stack,token_stack,errors| {
            webdriver_navigate(tl_stack, token_stack,errors);
        }}
        |
        [TokenType::IDENTIFIER(d_string()),TokenType::ASSIGN_OP] I_S
        {action:|tl_stack,token_stack,errors| {
            let name = get_input_from_token_stack!(token_stack.first()) ;
            let value = get_input_from_token_stack!(token_stack.last());

            let testcase = unwrap_or_return!(get_testcase_from_translator_stack(tl_stack));;
            token_stack.clear();
        }}
        |
        [TokenType::ACTION_CLICK,TokenType::STRING(d_string())]
        {error:"Please check teststeps syntax"}
        {action:|tl_stack,token_stack,errors| {
            element_click(tl_stack, token_stack,errors);
        }}
        |
        [TokenType::ACTION_BACK]
        {error:"Please check teststeps syntax"}
        {action:|tl_stack,token_stack,errors| {
            navigation_back(tl_stack, token_stack,errors);
        }}
        |
        [TokenType::ACTION_FORWARD]
        {action:|tl_stack,token_stack,errors| {
            navigation_forward(tl_stack, token_stack,errors);
        }}
        ;

        I_S -> [TokenType::IDENTIFIER(d_string())] | [TokenType::STRING(d_string())]
    );
    let mut parsed = SLR_Parser::new(gr.productions);
    parsed.compute_lr0_items();
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    parsed.parse(tt, &mut errors);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    execute();
}

fn get_testcase_from_translator_stack(tl_stack: &mut Vec<AST>) -> Option<&mut TestCase> {
    match tl_stack.get_mut(0) {
        Some(testcase) => match testcase {
            AST::TESTCASE(testcase_) => Some(testcase_),
            _ => None,
        },
        None => None,
    }
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.productionEnd)
        .for_each(|e| e.token.start = e.token.end);
}

fn webdriver_navigate(
    tl_stack: &mut Vec<AST>,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
    let testcase = unwrap_or_return!(get_testcase_from_translator_stack(tl_stack));
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
    testcase.insert_teststep(test_step);

    //clear token_stack after every use
    token_stack.clear();
}

fn element_click(
    tl_stack: &mut Vec<AST>,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
    let testcase = unwrap_or_return!(get_testcase_from_translator_stack(tl_stack));
    let locator = LocatorStrategy::parse(get_input_from_token_stack!(token_stack.last()));
    let test_step = TestStep::new(
        token_stack.first().unwrap().get_start_location(),
        token_stack.last().unwrap().get_end_location(),
        Class::ELEMENT,
        Method::ELEMENT(Element::CLICK),
        vec![Args::Locator(locator)],
    );
    testcase.insert_teststep(test_step);

    //clear token_stack after every use
    token_stack.clear();
}

fn navigation_back(
    tl_stack: &mut Vec<AST>,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
    let testcase = unwrap_or_return!(get_testcase_from_translator_stack(tl_stack));

    let test_step = TestStep::new(
        token_stack.first().unwrap().get_start_location(),
        token_stack.last().unwrap().get_end_location(),
        Class::NAVIGATION,
        Method::NAVIGATION(Navigation::BACK),
        vec![],
    );
    testcase.insert_teststep(test_step);

    //clear token_stack after every use
    token_stack.clear();
}

fn navigation_forward(
    tl_stack: &mut Vec<AST>,
    token_stack: &mut Vec<Token>,
    errors: &mut Vec<ParseError<Token>>,
) {
    let testcase = unwrap_or_return!(get_testcase_from_translator_stack(tl_stack));

    let test_step = TestStep::new(
        token_stack.first().unwrap().get_start_location(),
        token_stack.last().unwrap().get_end_location(),
        Class::NAVIGATION,
        Method::NAVIGATION(Navigation::FORWARD),
        vec![],
    );
    testcase.insert_teststep(test_step);

    //clear token_stack after every use
    token_stack.clear();
}
