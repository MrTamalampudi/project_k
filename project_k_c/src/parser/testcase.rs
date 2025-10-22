use manodae::error::ParseError;
use manodae::grammar;
use manodae::grammar::Grammar;
use manodae::parser::LR1_Parser;
use manodae::production::Production;
// use manodae::render_table::render;
use manodae::symbol::Symbol;
use std::rc::Rc;
use std::sync::Arc;

use super::Parser;
use crate::ast::testcase::TestCase;
use crate::class::{
    BinaryExpressionAction, CustomAction, LiteralExpressionAction, NavigationAction,
    TimeoutsAction, UnaryExpressionAction, ELEMENT,
};
use crate::class::{ElementAction, WebDriverAction};
use crate::engine::execute;
use crate::error_handler::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::actions::binary_expr::BinaryExpression;
use crate::parser::actions::custom::Custom;
use crate::parser::actions::driver::Driver;
use crate::parser::actions::element::Element;
use crate::parser::actions::literal_expression::LiteralExpression;
use crate::parser::actions::navigation::Navigation;
use crate::parser::actions::timeouts::Timeouts;
use crate::parser::actions::unary_expr::UnaryExpression;
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
    let d_num = || (1 as isize);
    let grammar: Grammar<TestCase, Token, TranslatorStack> = grammar!(
        Start -> Testcase Teststeps {error:"Testing"};

        Testcase -> [TokenType::TESTCASE];

        Teststeps ->Teststep | Teststep Teststeps;

        Teststep -> Navigate Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Driver::NAVIGATE(ast, token_stack, tl_stack, errors);
        }}
        |
        Click Expression
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,tl_stack,errors| {
            println!("checccccccccccccccccccccccccccccccck");
            Element::CLICK(ast,token_stack,tl_stack,errors);
        }}
        |
        Back
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::BACK(ast,token_stack,tl_stack,errors);
        }}
        |
        Forward
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::FORWARD(ast,token_stack,tl_stack,errors);
        }}
        |
        Refresh
        {action:|ast,token_stack,tl_stack,errors| {
            Navigation::REFRESH(ast,token_stack,tl_stack,errors);
        }}
        |
        Wait Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Timeouts::WAIT(ast,token_stack,tl_stack,errors);
        }}
        |
        Var Ident Assign Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Custom::VAR_DECLARATION(ast,token_stack,tl_stack,errors);
        }}
        |
        Assert Expression
        ;

        Getter ->
        Get Attribute Expression From Element Expression
        {action:|ast,token_stack,tl_stack,errors| {
                Element::GET_ATTRIBUTE(ast,token_stack,tl_stack,errors);
        }}
        |
        Get Current Url
        {action:|ast,token_stack,tl_stack,errors| {
                Driver::GET_CURRENT_URL(ast,token_stack,tl_stack,errors);
        }}
        |
        Get Title
        {action:|ast,token_stack,tl_stack,errors| {
                Driver::GET_TITLE(ast,token_stack,tl_stack,errors);
        }}
        ;

        Expression  -> LiteralExpression
        | BinaryExpression
        | UnaryExpression
        | Getter
        ;

        LiteralExpression ->
        Number
        {action:|ast,token_stack,tl_stack,errors| {
            LiteralExpression::NUMBER(ast, token_stack, tl_stack, errors);
        }}
        |
        String
        {action:|ast,token_stack,tl_stack,errors| {
            LiteralExpression::STRING(ast, token_stack, tl_stack, errors);
        }}
        |
        Ident
        {action:|ast,token_stack,tl_stack,errors| {
            LiteralExpression::IDENT(ast, token_stack, tl_stack, errors);
        }}
        |
        True
        {action:|ast,token_stack,tl_stack,errors| {
            LiteralExpression::BOOLEAN(ast, token_stack, tl_stack, errors);
        }}
        |
        False
        {action:|ast,token_stack,tl_stack,errors| {
            LiteralExpression::BOOLEAN(ast, token_stack, tl_stack, errors);
        }}
        ;

        UnaryExpression -> NegationExpression;

        NegationExpression ->
        Negation GroupedExpression
        {action:|ast,token_stack,tl_stack,errors| {
            UnaryExpression::NEGATION(ast, token_stack, tl_stack, errors);
        }}
        ;

        GroupedExpression ->
        Left_paran Expression Right_paran
        {action:|ast,token_stack,tl_stack,errors| {
            UnaryExpression::GROUPED(ast, token_stack, tl_stack, errors);
        }}
        ;

        BinaryExpression ->
        ComparisionExpression
        |
        ArthimaticExpression
        |
        LogicalExpression
        ;

        LogicalExpression ->
        Expression And Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::AND(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Or Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::OR(ast, token_stack, tl_stack, errors);
        }}
        ;

        ComparisionExpression ->
        // Expression Equality Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::EQ(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Not_equal Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::NE(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Greater_than Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::GT(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Lesser_than Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::LT(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Greater_than_equal Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::GE(ast, token_stack, tl_stack, errors);
        // }}
        // |
        Expression Lesser_than_equal Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::LE(ast, token_stack, tl_stack, errors);
        }}
        ;

        ArthimaticExpression ->
        Expression Plus Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::ADD(ast, token_stack, tl_stack, errors);
        }}
        // |
        // Expression Minus Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::SUB(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Expression // special case where 1-1 here we need to number + number
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::SPL_SUB(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Multiply Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::MUL(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Forward_slash Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::DIV(ast, token_stack, tl_stack, errors);
        // }}
        // |
        // Expression Modulus Expression
        // {action:|ast,token_stack,tl_stack,errors| {
        //     BinaryExpression::REM(ast, token_stack, tl_stack, errors);
        // }}
        ;

        //Actions
        Navigate            -> [TokenType::NAVIGATE];
        Click               -> [TokenType::CLICK];
        Back                -> [TokenType::BACK];
        Forward             -> [TokenType::FORWARD];
        Refresh             -> [TokenType::REFRESH];
        Get                 -> [TokenType::GET];
        Wait                -> [TokenType::WAIT];
        Assert              -> [TokenType::ASSERT];

        //Nouns
        Attribute           -> [TokenType::ATTRIBUTE];
        Element             -> [TokenType::ELEMENT];
        Url                 -> [TokenType::URL];
        Title               -> [TokenType::TITLE];
        Var                 -> [TokenType::VAR];

        //Prepositions
        From                -> [TokenType::FROM];
        To                  -> [TokenType::TO];

        //Adjectives
        Current             -> [TokenType::CURRENT];

        //Operators
        Assign              -> [TokenType::ASSIGN_OP];
        Negation            -> [TokenType::NEGATION];
        Plus                -> [TokenType::PLUS];
        Minus               -> [TokenType::MINUS];
        Multiply            -> [TokenType::MULTIPLY];
        Forward_slash       -> [TokenType::FORWARDSLASH];
        Modulus             -> [TokenType::MODULUS];
        Equality            -> [TokenType::EQUALITY];
        Not_equal           -> [TokenType::NOT_EQUAL];
        Greater_than        -> [TokenType::GREATER_THAN];
        Lesser_than         -> [TokenType::LESSER_THAN];
        Greater_than_equal  -> [TokenType::GREATER_THAN_EQUAL_TO];
        Lesser_than_equal   -> [TokenType::LESSER_THAN_EQUAL_TO];

        //Conjunctions
        And                 -> [TokenType::AND];
        Or                  -> [TokenType::OR];

        //chars
        Left_paran          -> [TokenType::LEFT_PARAN];
        Right_paran         -> [TokenType::RIGHT_PARAN];

        //Inputs
        String              -> [TokenType::STRING(d_string())];
        Ident               -> [TokenType::IDENTIFIER(d_string())];
        Number              -> [TokenType::NUMBER(d_num())];

        //Boolean
        True               -> [TokenType::TRUE];
        False              -> [TokenType::FALSE];
    );
    let mut lalr_parser = LR1_Parser::new(grammar);
    lalr_parser.construct_LALR_Table();
    // lalr_parser
    //     .action
    //     .keys()
    //     .into_iter()
    //     .for_each(|state| println!("state :{:?}", state.index));
    // render(&lalr_parser);
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    let mut ast: TestCase = TestCase::new();
    lalr_parser.parse(tt, &mut errors, &mut ast);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    parser.ctx.program = Program {
        testcase: ast.clone(),
    };
    if parser.ctx.errors.errors.is_empty() {
        execute(&mut parser.ctx.program.testcase);
    } else {
        log::error!("Errors {:#?}", parser.ctx.errors.errors);
    }
    log::info!("variables {:#?}", parser.ctx.program.testcase.variables);
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.production_end)
        .for_each(|e| e.token.span.start = e.token.span.end);
}
