use manodae::error::ParseError;

use crate::location::SpanTrait;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
use crate::{
    ast::{
        expression::{BinOpKind, ExpKind, Expr},
        testcase::TestCase,
    },
    class::BinaryExpressionAction,
    parser::{errors::EXPECT_EXPR, translator_stack::TranslatorStack},
    token::Token,
};

pub struct BE;

impl BinaryExpressionAction for BE {
    //expr + expr
    fn ADD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let plus_token = _token_stack.pop().unwrap(); //pop 'plus' token

        let expr2_tl = _tl_stack.pop().unwrap();
        let expr2 = if let TranslatorStack::Expression(expr) = expr2_tl {
            expr
        } else {
            _errors.push_error(&plus_token, &expr2_tl.get_span(), EXPECT_EXPR.to_string());
            return;
        };

        let expr1_tl = _tl_stack.pop().unwrap();
        let expr1 = if let TranslatorStack::Expression(expr) = expr1_tl {
            expr
        } else {
            _errors.push_error(&plus_token, &expr1_tl.get_span(), EXPECT_EXPR.to_string());
            return;
        };

        let span = expr1.span.to(&expr2.span);
        if expr1.primitive != expr2.primitive {
            _errors.push_error(&plus_token, &span, MISMATCHED_TYPES.to_string());
            return;
        }
        let primitive = expr1.primitive.clone();
        let kind = ExpKind::Binary(BinOpKind::Add, Box::new(expr1), Box::new(expr2));
        let expr = Expr {
            kind,
            span,
            primitive,
        };
        _tl_stack.push(TranslatorStack::Expression(expr));
    }
    fn SUB(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn SPL_SUB(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn MUL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn DIV(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn REM(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn AND(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn OR(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn EQ(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn NE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn LT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn LE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn GE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn GT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
}
