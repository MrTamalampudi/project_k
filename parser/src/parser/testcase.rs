use std::rc::Rc;
use std::time::Instant;

use super::Parser;
use crate::error_handler::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::TokenType;
use crate::parser::actions::binary_expr::BinaryExpression;
use crate::parser::actions::conditional::Conditional;
use crate::parser::actions::custom::Custom;
use crate::parser::actions::driver::Driver;
use crate::parser::actions::element::Element;
use crate::parser::actions::literal_expression::LiteralExpression;
use crate::parser::actions::navigation::Navigation;
use crate::parser::actions::shared::Shared;
use crate::parser::actions::timeouts::Timeouts;
use crate::parser::actions::unary_expr::UnaryExpression;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::program::Program;
use crate::token::Token;
use ast::testcase::TestCase;
use ast::teststep::Body;
use class::{
    BinaryExpressionAction, ConditionalStmtAction, CustomAction, ElementAction,
    LiteralExpressionAction, NavigationAction, TimeoutsAction, UnaryExpressionAction,
    WebDriverAction,
};
use log::debug;
use manodae::prelude::*;

//generates error if there are no parser generated files
include!("./parser_generated/parser.rs");

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
    let tt: Vec<Token> = parser.lexer.tokens.iter().cloned().collect();
    let d_string = || "".to_string();
    let d_num = || (1 as isize);
    let time = Instant::now();
    let grammar: Grammar<TestCase, Token, TranslatorStack> = grammar!(
        Start -> Testcase Newlines Teststeps {error:"Testing"}
        {action:|ast,token_stack,tl_stack,errors| {
            Shared::set_body(ast, tl_stack.pop_body());
        }}
        ;

        Newlines -> Newline | Newline Newlines;

        Testcase -> [TokenType::TESTCASE]
        {action:|ast,token_stack,tl_stack,errors| {
            tl_stack.push(TranslatorStack::Body(Body::new()));
        }}
        ;

        Teststeps ->Teststep Newlines | Teststep Newlines Teststeps;

        Teststep -> Navigate Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Driver::NAVIGATE(ast, token_stack, tl_stack, errors);
        }}
        |Close
        {action:|ast,token_stack,tl_stack,errors| {
            Driver::CLOSE(ast, token_stack, tl_stack, errors);
        }}
        |
        Click Expression
        {error:"Please check teststeps syntax"}
        {action:|ast,token_stack,tl_stack,errors| {
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
        Ident Assign Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Custom::VAR_DECLARATION(ast,token_stack,tl_stack,errors);
        }}
        |
        Assert Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Custom::ASSERT(ast,token_stack,tl_stack,errors);
        }}
        |
        Enter Expression In Element Expression
        {action:|ast,token_stack,tl_stack,errors| {
            Element::SENDKEYS(ast,token_stack,tl_stack,errors);
        }}
        |
        IfStmt
        ;

        // ***** Conditional statement *****
        IfStmt -> IfExpr
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::IF(ast,token_stack,tl_stack,errors);
        }}
        | IfExpr ElseIfStmt
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::IF(ast,token_stack,tl_stack,errors);
        }}
        | IfExpr ElseExpr
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::IF(ast,token_stack,tl_stack,errors);
        }}
        ;

        ElseIfStmt -> ElseIfExpr
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::ELSE_IF(ast,token_stack,tl_stack,errors);
        }}
        | ElseIfExpr ElseIfStmt
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::ELSE_IF(ast,token_stack,tl_stack,errors);
        }}
        | ElseIfExpr ElseExpr
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::ELSE_IF(ast,token_stack,tl_stack,errors);
        }}
        ;

        IfExpr -> If Expression L_CurlyBrace Newline Teststeps R_CurlyBrace;

        ElseIfExpr-> Else If Expression L_CurlyBrace Newline Teststeps R_CurlyBrace;

        ElseExpr -> Else L_CurlyBrace Newline Teststeps R_CurlyBrace
        {action:|ast,token_stack,tl_stack,errors| {
            Conditional::ELSE(ast,token_stack,tl_stack,errors);
        }}
        ;

        // *****

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

        UnaryExpression -> NegationExpression | GroupedExpression;

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
        Expression Equality Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::EQ(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Not_equal Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::NE(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Greater_than Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::GT(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Lesser_than Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::LT(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Greater_than_equal Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::GE(ast, token_stack, tl_stack, errors);
        }}
        |
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
        |
        Expression Minus Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::SUB(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Multiply Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::MUL(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Forward_slash Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::DIV(ast, token_stack, tl_stack, errors);
        }}
        |
        Expression Modulus Expression
        {action:|ast,token_stack,tl_stack,errors| {
            BinaryExpression::REM(ast, token_stack, tl_stack, errors);
        }}
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
        Enter               -> [TokenType::ENTER];
        Close               -> [TokenType::CLOSE];

        //Nouns
        Attribute           -> [TokenType::ATTRIBUTE];
        Element             -> [TokenType::ELEMENT];
        Url                 -> [TokenType::URL];
        Title               -> [TokenType::TITLE];
        Var                 -> [TokenType::VAR];

        //Prepositions
        From                -> [TokenType::FROM];
        To                  -> [TokenType::TO];
        In                  -> [TokenType::IN];

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
        If                  -> [TokenType::IF];
        Else                -> [TokenType::ELSE];

        //chars
        Left_paran          -> [TokenType::LEFT_PARAN];
        Right_paran         -> [TokenType::RIGHT_PARAN];
        L_CurlyBrace        -> [TokenType::L_CURLY_BRACE]
        // L_CURLY_BRACE means starting of a block
        // so we are keeping this action to add new body to tl_stack
        {action:|ast,token_stack,tl_stack,errors| {
            tl_stack.push(TranslatorStack::Body(Body::new()));
        }};
        R_CurlyBrace        -> [TokenType::R_CURLY_BRACE];

        //Inputs
        String              -> [TokenType::STRING(d_string())];
        Ident               -> [TokenType::IDENTIFIER(d_string())];
        Number              -> [TokenType::NUMBER(d_num())];

        //Boolean
        True               -> [TokenType::TRUE];
        False              -> [TokenType::FALSE];

        Newline            -> [TokenType::NEW_LINE];
    );
    let els = time.elapsed();
    debug!("grammar macro expansion time {:#?}", els);
    let time = Instant::now();
    let path = std::env::current_dir()
        .unwrap()
        .join("..")
        .join(file! {})
        .parent()
        .unwrap()
        .to_path_buf();
    Codegen::gen(path, grammar, ["TestCase", "Token", "TranslatorStack"]);
    let els = time.elapsed();
    debug!("codegeneration time {:#?}", els);
    // render(&lalr_parser);
    let mut errors: Vec<ParseError<Token>> = Vec::new();
    let mut ast: TestCase = TestCase::new();
    let time = Instant::now();
    get_parser().parse(tt, &mut errors, &mut ast);
    let els = time.elapsed();
    debug!("parsing time {:#?}", els);
    refine_errors(&mut errors);
    let transformed_errors: Vec<ErrorInfo> = errors
        .iter()
        .map(|e| parse_error_to_error_info(e.clone()))
        .collect();
    parser.ctx.errors.errors.extend(transformed_errors);
    parser.ctx.ast = Program {
        testcase: ast.clone(),
    };
    log::info!("variables {:#?}", parser.ctx.ast.testcase.variables);
}

fn refine_errors(errors: &mut Vec<ParseError<Token>>) {
    errors
        .iter_mut()
        .filter(|e| e.production_end)
        .for_each(|e| e.token.span.start = e.token.span.end);
}
