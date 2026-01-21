use std::rc::Rc;
use std::time::Instant;

use super::Parser;
use crate::error_handler::{parse_error_to_error_info, ErrorInfo};
use crate::keywords::NTokenType;
use crate::parser::actions::{
    BinaryExpression, ControlFlow, Custom, Driver, Element, Getter, LiteralExpression, Navigation,
    Shared, Timeouts, UnaryExpression,
};
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::program::Program;
use ast::Body;
use ast::TestCase;
use class::{
    BinaryExpressionAction, ControlFlowAction, CustomAction, ElementAction, GetterAction,
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
    let d_string = || "".to_string();
    let d_num = || (1 as isize);
    let time = Instant::now();
    let grammar: Grammar<TestCase, NTokenType, TranslatorStack> = grammar!(
        Start -> Testcase Teststeps
        { |ast,token_stack,tl_stack,errors| Shared::set_body(ast, tl_stack.pop_body()) };

        [non_terminal_productions]

        Teststeps ->Teststep
        | Teststep Teststeps;

        Teststep -> DriverActions
        | NavigationActions
        | ElementActions
        | TimeoutActions
        | CustomActions
        | ControlFlow
        ;

        // ### Driver ###
        // Actions
        DriverActions   -> NAVIGATE
        | CLOSE
        ;

        NAVIGATE -> Navigate Expression
        {|ast,token_stack,tl_stack,errors| Driver::NAVIGATE(ast, token_stack, tl_stack, errors) };

        CLOSE -> Close
        {|ast,token_stack,tl_stack,errors| Driver::CLOSE(ast, token_stack, tl_stack, errors) };

        // ### Navigation ###
        NavigationActions -> BACK
        | FORWARD
        | REFRESH ;

        BACK -> Back
        { |ast,token_stack,tl_stack,errors| Navigation::BACK(ast,token_stack,tl_stack,errors) };

        FORWARD -> Forward
        { |ast,token_stack,tl_stack,errors| Navigation::FORWARD(ast,token_stack,tl_stack,errors) };
        REFRESH -> Refresh
        { |ast,token_stack,tl_stack,errors| Navigation::REFRESH(ast,token_stack,tl_stack,errors) };

        // ### Element ###
        ElementActions -> CLICK
        | SENDKEYS ;

        CLICK ->Click Expression
        { |ast,token_stack,tl_stack,errors| Element::CLICK(ast,token_stack,tl_stack,errors) };

        SENDKEYS -> Enter Expression In Element Expression
        {|ast,token_stack,tl_stack,errors| Element::SENDKEYS(ast,token_stack,tl_stack,errors) };

        // ### Timeouts ###
        TimeoutActions -> WAIT;

        WAIT -> Wait Expression
        { |ast,token_stack,tl_stack,errors| Timeouts::WAIT(ast,token_stack,tl_stack,errors) };

        // ### Custom ###
        CustomActions -> VAR_DECLARATION
        | ASSERT ;

        VAR_DECLARATION -> Ident Assign Expression
        { |ast,token_stack,tl_stack,errors| Custom::VAR_DECLARATION(ast,token_stack,tl_stack,errors) };

        ASSERT -> Assert Expression
        { |ast,token_stack,tl_stack,errors| Custom::ASSERT(ast,token_stack,tl_stack,errors) };

        ControlFlow -> IfStmt
        | WhileStmt
        | ForLoop;

        // ***** Control flow statement *****
        IfStmt -> IfExpr
        { |ast,token_stack,tl_stack,errors| ControlFlow::IF(ast,token_stack,tl_stack,errors) }
        | IfExpr ElseIfStmt
        { |ast,token_stack,tl_stack,errors| ControlFlow::IF(ast,token_stack,tl_stack,errors) }
        | IfExpr ElseExpr
        { |ast,token_stack,tl_stack,errors| ControlFlow::IF(ast,token_stack,tl_stack,errors) };

        ElseIfStmt -> ElseIfExpr
        { |ast,token_stack,tl_stack,errors| ControlFlow::ELSE_IF(ast,token_stack,tl_stack,errors) }
        | ElseIfExpr ElseIfStmt
        { |ast,token_stack,tl_stack,errors| ControlFlow::ELSE_IF(ast,token_stack,tl_stack,errors) }
        | ElseIfExpr ElseExpr
        { |ast,token_stack,tl_stack,errors| ControlFlow::ELSE_IF(ast,token_stack,tl_stack,errors) };

        IfExpr -> If Expression L_CurlyBrace Teststeps R_CurlyBrace;

        ElseIfExpr-> Else If Expression L_CurlyBrace Teststeps R_CurlyBrace;

        ElseExpr -> Else L_CurlyBrace Teststeps R_CurlyBrace
        { |ast,token_stack,tl_stack,errors| ControlFlow::ELSE(ast,token_stack,tl_stack,errors) };

        WhileStmt -> While Expression L_CurlyBrace Teststeps R_CurlyBrace
        { |ast,token_stack,tl_stack,errors| ControlFlow::WHILE(ast,token_stack,tl_stack,errors) };

        ForLoop -> For ForLoopHelper L_CurlyBrace Teststeps R_CurlyBrace
        { |ast,token_stack,tl_stack,errors| ControlFlow::FOR(ast,token_stack,tl_stack,errors) };

        ForLoopHelper -> Ident In Expression
        { |ast,token_stack,tl_stack,errors| ControlFlow::HELPER(ast,token_stack,tl_stack,errors) };
        // *****

        Getter -> GET_TITLE
        | GET_CURRENT_URL
        | GET_ATTRIBUTE
        | IS_DISPLAYED
        | IS_ENABLED
        | IS_SELECTED
        | GET_TEXT
        | GET_CSS_VALUE
        | GET_TAG_NAME
        ;

        GET_TITLE -> Get Title
        {|ast,token_stack,tl_stack,errors| Getter::GET_TITLE(ast,token_stack,tl_stack,errors)};

        GET_CURRENT_URL -> Get Current Url
        {|ast,token_stack,tl_stack,errors| Getter::GET_CURRENT_URL(ast,token_stack,tl_stack,errors) };

        GET_ATTRIBUTE -> Get Attribute Expression From Element Expression
        { |ast,token_stack,tl_stack,errors| Getter::GET_ATTRIBUTE(ast,token_stack,tl_stack,errors) };

        IS_DISPLAYED -> Is Element Expression Displayed
        { |ast,token_stack,tl_stack,errors| Getter::IS_DISPLAYED(ast,token_stack,tl_stack,errors) };

        IS_ENABLED -> Is Element Expression Enabled
        { |ast,token_stack,tl_stack,errors| Getter::IS_ENABLED(ast,token_stack,tl_stack,errors) };

        IS_SELECTED -> Is Element Expression Selected
        { |ast,token_stack,tl_stack,errors| Getter::IS_SELECTED(ast,token_stack,tl_stack,errors) };

        GET_TEXT -> Get Text From Element Expression
        { |ast,token_stack,tl_stack,errors| Getter::GET_TEXT(ast,token_stack,tl_stack,errors) };

        GET_CSS_VALUE -> Get Css Value Expression From Element Expression
        { |ast,token_stack,tl_stack,errors| Getter::GET_CSS_VALUE(ast,token_stack,tl_stack,errors) };

        GET_TAG_NAME -> Get Tag Name From Element Expression
        { |ast,token_stack,tl_stack,errors| Getter::GET_TAG_NAME(ast,token_stack,tl_stack,errors) };

        Expression  -> LiteralExpression
        | BinaryExpression
        | UnaryExpression
        | ArrayExpression
        { |ast,token_stack,tl_stack,errors| LiteralExpression::ARRAY(ast, token_stack, tl_stack, errors) }
        | Getter;

        LiteralExpression -> Number
        { |ast,token_stack,tl_stack,errors| LiteralExpression::NUMBER(ast, token_stack, tl_stack, errors) }
        | String
        { |ast,token_stack,tl_stack,errors| LiteralExpression::STRING(ast, token_stack, tl_stack, errors) }
        | Ident
        { |ast,token_stack,tl_stack,errors| LiteralExpression::IDENT(ast, token_stack, tl_stack, errors) }
        | Boolean
        { |ast,token_stack,tl_stack,errors| LiteralExpression::BOOLEAN(ast, token_stack, tl_stack, errors) };

        UnaryExpression -> NegationExpression
        | GroupedExpression ;

        NegationExpression -> Negation GroupedExpression
        { |ast,token_stack,tl_stack,errors| UnaryExpression::NEGATION(ast, token_stack, tl_stack, errors) };

        GroupedExpression -> Left_paran Expression Right_paran
        { |ast,token_stack,tl_stack,errors| UnaryExpression::GROUPED(ast, token_stack, tl_stack, errors) };

        BinaryExpression -> ComparisionExpression
        | ArthimaticExpression
        | LogicalExpression;

        LogicalExpression -> Expression And Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::AND(ast, token_stack, tl_stack, errors) }
        | Expression Or Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::OR(ast, token_stack, tl_stack, errors) };

        ComparisionExpression -> Expression Equality Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::EQ(ast, token_stack, tl_stack, errors) }
        | Expression Not_equal Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::NE(ast, token_stack, tl_stack, errors) }
        | Expression Greater_than Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::GT(ast, token_stack, tl_stack, errors) }
        | Expression Lesser_than Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::LT(ast, token_stack, tl_stack, errors) }
        | Expression Greater_than_equal Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::GE(ast, token_stack, tl_stack, errors) }
        | Expression Lesser_than_equal Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::LE(ast, token_stack, tl_stack, errors) } ;

        ArthimaticExpression -> Expression Plus Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::ADD(ast, token_stack, tl_stack, errors) }
        | Expression Minus Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::SUB(ast, token_stack, tl_stack, errors) }
        | Expression Multiply Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::MUL(ast, token_stack, tl_stack, errors) }
        | Expression Forward_slash Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::DIV(ast, token_stack, tl_stack, errors) }
        | Expression Modulus Expression
        { |ast,token_stack,tl_stack,errors| BinaryExpression::REM(ast, token_stack, tl_stack, errors) } ;

        ArrayExpression -> L_SquareBrace R_SquareBrace
        | L_SquareBrace ArrayElements R_SquareBrace;

        ArrayElements -> Expression
        | Expression Comma ArrayElements
        { |ast,token_stack,tl_stack,errors| {
            //pop comma token
            token_stack.pop();
        }};

        [terminal_productions]

        Testcase -> [NTokenType::TESTCASE]
        { |ast,token_stack,tl_stack,errors| tl_stack.push(TranslatorStack::Body(Body::new())) };

        //Actions
        Navigate            -> [NTokenType::NAVIGATE];
        Click               -> [NTokenType::CLICK];
        Back                -> [NTokenType::BACK];
        Forward             -> [NTokenType::FORWARD];
        Refresh             -> [NTokenType::REFRESH];
        Get                 -> [NTokenType::GET];
        Wait                -> [NTokenType::WAIT];
        Assert              -> [NTokenType::ASSERT];
        Enter               -> [NTokenType::ENTER];
        Close               -> [NTokenType::CLOSE];

        //Nouns
        Attribute           -> [NTokenType::ATTRIBUTE];
        Element             -> [NTokenType::ELEMENT];
        Url                 -> [NTokenType::URL];
        Title               -> [NTokenType::TITLE];
        Css                 -> [NTokenType::CSS];
        Value               -> [NTokenType::VALUE];
        Text                -> [NTokenType::TEXT];
        Tag                 -> [NTokenType::TAG];
        Name                -> [NTokenType::NAME];

        //Prepositions
        From                -> [NTokenType::FROM];
        To                  -> [NTokenType::TO];
        In                  -> [NTokenType::IN];
        Is                  -> [NTokenType::IS];

        //Adjectives
        Current             -> [NTokenType::CURRENT];
        Displayed           -> [NTokenType::DISPLAYED];
        Enabled             -> [NTokenType::ENABLED];
        Selected            -> [NTokenType::SELECTED];

        //Operators
        Assign              -> [NTokenType::ASSIGN_OP];
        Negation            -> [NTokenType::NEGATION];
        Plus                -> [NTokenType::PLUS];
        Minus               -> [NTokenType::MINUS];
        Multiply            -> [NTokenType::MULTIPLY];
        Forward_slash       -> [NTokenType::FORWARDSLASH];
        Modulus             -> [NTokenType::MODULUS];
        Equality            -> [NTokenType::EQUALITY];
        Not_equal           -> [NTokenType::NOT_EQUAL];
        Greater_than        -> [NTokenType::GREATER_THAN];
        Lesser_than         -> [NTokenType::LESSER_THAN];
        Greater_than_equal  -> [NTokenType::GREATER_THAN_EQUAL_TO];
        Lesser_than_equal   -> [NTokenType::LESSER_THAN_EQUAL_TO];

        //Punctuations
        Comma               -> [NTokenType::COMMA];

        //Conjunctions
        And                 -> [NTokenType::AND];
        Or                  -> [NTokenType::OR];
        If                  -> [NTokenType::IF];
        Else                -> [NTokenType::ELSE];
        While               -> [NTokenType::WHILE];
        For                 -> [NTokenType::FOR];

        //chars
        Left_paran          -> [NTokenType::LEFT_PARAN];
        Right_paran         -> [NTokenType::RIGHT_PARAN];
        L_CurlyBrace        -> [NTokenType::L_CURLY_BRACE]
        // L_CURLY_BRACE means starting of a block
        // so we are keeping this action to add new body to tl_stack
        { |ast,token_stack,tl_stack,errors| tl_stack.push(TranslatorStack::Body(Body::new())) };

        R_CurlyBrace        -> [NTokenType::R_CURLY_BRACE];
        L_SquareBrace       -> [NTokenType::L_SQUARE_BRACE]
        { |ast,token_stack,tl_stack,errors| tl_stack.push(TranslatorStack::ArrayDelim) };

        R_SquareBrace       -> [NTokenType::R_SQUARE_BRACE];

        //Inputs
        String              -> [NTokenType::STRING(String::new())];
        Ident               -> [NTokenType::IDENTIFIER(String::new())];
        Number              -> [NTokenType::NUMBER(1.0)];

        //Boolean
        Boolean            -> [NTokenType::BOOL(true)];
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
    Codegen::gen(path, grammar, ["TestCase", "NTokenType", "TranslatorStack"]);
    let els = time.elapsed();
    debug!("codegeneration time {:#?}", els);
    // render(&lalr_parser);
    let mut errors: Vec<ParseError> = Vec::new();
    let mut ast: TestCase = TestCase::new();
    let time = Instant::now();
    get_parser().parse(parser.lexer.clone(), &mut errors, &mut ast);
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
}

fn refine_errors(errors: &mut Vec<ParseError>) {
    errors
        .iter_mut()
        .filter(|e| e.production_end)
        .for_each(|e| e.span.start = e.span.end);
}
