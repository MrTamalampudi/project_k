use log::debug;
use slr_parser::error::ParseError;
use slr_parser::grammar;
use slr_parser::grammar::Grammar;
use slr_parser::parser::Parser as SLR_Parser;
use slr_parser::production::Production;
use slr_parser::symbol::Symbol;
use slr_parser::terminal::Terminal;
use std::sync::Arc;

use super::Parser;
use crate::ast::testcase::TestCase;
use crate::class::{CustomAction, NavigationAction};
use crate::class::{ElementAction, WebDriverAction};
use crate::engine::execute;
use crate::error_handler::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::actions::custom::Custom;
use crate::parser::actions::driver::Driver;
use crate::parser::actions::element::Element;
use crate::parser::actions::navigation::Navigation;
use crate::parser::translator_stack::TranslatorStack;
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
    let grammar: Grammar<TestCase, Token, TranslatorStack> = grammar!(
        Start -> Testcase Teststeps {error:"Testing"};

        Testcase -> [TokenType::TESTCASE];

        Teststeps -> Teststep
        {error:"Teststeps body_"}
        | Teststep Teststeps
        {error:"Teststeps body_ 2"};

        Teststep -> navigate string
        {error:"Expected syntax ' navigate \"url\" '"}
        {action:|ast,token_stack,tl_stack,errors| {
            Driver::NAVIGATE(ast,token_stack,tl_stack,errors);
        }}
        |
        click string
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,tl_stack,errors| {
            Element::CLICK(ast,token_stack,tl_stack,errors);
        }}
        |
        back
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::BACK(ast,token_stack,tl_stack,errors);
        }}
        |
        forward
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::FORWARD(ast,token_stack,tl_stack,errors);
        }}
        |
        refresh
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::REFRESH(ast,token_stack,tl_stack,errors);
        }}
        |
        VAR_DECLARATION
        ;

        VAR_DECLARATION -> ident assign VAR_RHS
        {action:|ast,token_stack,tl_stack,errors| {
                Custom::VAR_DECLARATION(ast,token_stack,tl_stack,errors);
        }}
        ;
        VAR_RHS -> IDENT_OR_STRING | GETTER;
        GETTER -> get attribute IDENT_OR_STRING from element IDENT_OR_STRING
        {action:|ast,token_stack,tl_stack,errors| {
                Element::GET_ATTRIBUTE(ast,token_stack,tl_stack,errors);
        }}
        |
        get current url
        {action:|ast,token_stack,tl_stack,errors| {
                Driver::GET_CURRENT_URL(ast,token_stack,tl_stack,errors);
        }}
        ;


        IDENT_OR_STRING -> ident | string;

        //Actions
        navigate    -> [TokenType::NAVIGATE];
        click       -> [TokenType::CLICK];
        back        -> [TokenType::BACK];
        forward     -> [TokenType::FORWARD];
        refresh     -> [TokenType::REFRESH];
        get         -> [TokenType::GET];

        //Nouns
        attribute   -> [TokenType::ATTRIBUTE];
        element     -> [TokenType::ELEMENT];
        url         -> [TokenType::URL];

        //Prepositions
        from        -> [TokenType::FROM];
        to          -> [TokenType::TO];

        //Adjectives
        current     -> [TokenType::CURRENT];

        //Operators
        assign      -> [TokenType::ASSIGN_OP];

        //Inputs
        string      -> [TokenType::STRING(d_string())];
        ident       -> [TokenType::IDENTIFIER(d_string())];
    );
    let mut slr_parser = SLR_Parser::new(grammar.productions);
    slr_parser.compute_lr0_items();
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    let mut ast: TestCase = TestCase::new();
    slr_parser.parse(tt, &mut errors, &mut ast);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    parser.ctx.program = Program {
        testcase: ast.clone(),
    };
    execute(&mut parser.ctx.program.testcase);
    log::info!("variables {:#?}", parser.ctx.program.testcase.variables);
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.production_end)
        .for_each(|e| e.token.start = e.token.end);
}
