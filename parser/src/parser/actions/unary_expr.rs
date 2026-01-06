use class::UnaryExpressionAction;
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;

use crate::{
    a_types,
    parser::{
        errors::_INVALID_NEGATION_EXPR_USE,
        errorss::ActionError,
        translator_stack::{TLVec, TranslatorStack},
    },
    token::Token,
};
use ast::{
    expression::{ExpKind, Expr, UnOp},
    Primitives, TestCase,
};

pub struct UnaryExpression;

impl UnaryExpressionAction for UnaryExpression {
    a_types!();
    // (expr)
    #[pop_token(_left_brace, _right_brace)]
    fn GROUPED(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }

    // !expr
    #[pop_token(negation_token)]
    #[pop_expr(expr)]
    fn NEGATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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
