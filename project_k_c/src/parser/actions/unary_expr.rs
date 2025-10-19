use manodae::error::ParseError;

use crate::{
    ast::{
        expression::{ExpKind, Expr, UnOp},
        primitives::Primitives,
        testcase::TestCase,
    },
    class::UnaryExpressionAction,
    parser::{
        errors::_INVALID_NEGATION_EXPR_USE,
        errorss::ActionError,
        translator_stack::{TLVec, TranslatorStack},
    },
    token::Token,
};

pub struct UnaryExpression;

impl UnaryExpressionAction for UnaryExpression {
    // (expr)
    fn GROUPED(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        _token_stack.pop(); // pop "(" token
        _token_stack.pop(); // pop ")" token
    }

    // !expr
    fn NEGATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let negation_token = _token_stack.pop().unwrap(); // pop "!" token
        let expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&negation_token, &span, error);
                return;
            }
        };

        if expr.boolean() {
            let expr_ = Expr {
                primitive: Primitives::Boolean,
                span: negation_token.span.to(&expr.span),
                kind: ExpKind::Unary(UnOp::Not, Box::new(expr)),
            };
            _tl_stack.push_expr(expr_);
        } else {
            _errors.push_error(
                &negation_token,
                &expr.span,
                _INVALID_NEGATION_EXPR_USE.to_string(),
            );
        }
    }
}
