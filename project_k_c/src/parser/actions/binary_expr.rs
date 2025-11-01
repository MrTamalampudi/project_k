use manodae::error::ParseError;

use crate::ast::primitives::Primitives;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::TLVec;
use crate::{
    ast::{
        expression::{BinOpKind, ExpKind, Expr},
        testcase::TestCase,
    },
    class::BinaryExpressionAction,
    parser::translator_stack::TranslatorStack,
    token::Token,
};

pub struct BinaryExpression;

impl BinaryExpression {
    fn common(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
        op: BinOpKind,
    ) {
        let token = _token_stack.pop().unwrap(); //pop 'op' token

        let expr2 = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&token, &span, error);
                return;
            }
        };

        let expr1 = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&token, &span, error);
                return;
            }
        };

        let span = expr1.span.to(&expr2.span);

        //typechecking
        if expr1.primitive != expr2.primitive {
            _errors.push_error(&token, &span, MISMATCHED_TYPES.to_string());
            return;
        }

        let primitive = if op.is_bool_op() {
            Primitives::Boolean
        } else {
            expr1.primitive.clone()
        };
        let kind = ExpKind::Binary(op, Box::new(expr1), Box::new(expr2));
        let expr = Expr {
            kind,
            span,
            primitive,
        };
        _tl_stack.push(TranslatorStack::Expression(expr));
    }
}

impl BinaryExpressionAction for BinaryExpression {
    //expr + expr
    fn ADD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Add);
    }
    fn SUB(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Sub);
    }
    fn MUL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Mul);
    }
    fn DIV(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Div);
    }
    fn REM(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Rem);
    }
    fn AND(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::And);
    }
    fn OR(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Or);
    }
    fn EQ(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Eq);
    }
    fn NE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Ne);
    }
    fn LT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Lt);
    }
    fn LE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Le);
    }
    fn GE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Ge);
    }
    fn GT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Gt);
    }
}
