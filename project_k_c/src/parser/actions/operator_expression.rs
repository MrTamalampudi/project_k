use manodae::error::ParseError;

use crate::{
    ast::{
        expression::{ExpKind, Expr, Literal, UnOp},
        testcase::TestCase,
    },
    class::UnaryExpressionAction,
    parser::{errors::_INVALID_NEGATION_EXPR_USE, translator_stack::TranslatorStack},
    token::Token,
};

pub struct UnaryExpression {}

impl UnaryExpressionAction for UnaryExpression {
    fn NEGATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let expr_tl = _tl_stack.pop().unwrap();
        let negation_token = _token_stack.pop().unwrap();
        if let TranslatorStack::Expression(_expr) = expr_tl {
            match &_expr.kind {
                ExpKind::Binary(binop, _, _) => {
                    if binop.is_bool_op() {
                        let expr = Expr {
                            kind: ExpKind::Unary(UnOp::Not, Box::new(_expr.clone())),
                            span: negation_token.span.to(&_expr.span),
                        };
                        _tl_stack.push(TranslatorStack::Expression(expr));
                    } else {
                        negation_token.span.to(&_expr.span);
                        _errors.push(ParseError {
                            token: negation_token,
                            message: String::from(_INVALID_NEGATION_EXPR_USE),
                            production_end: false,
                        });
                    }
                }
                ExpKind::Unary(unop, _) => {
                    if &UnOp::Not == unop {
                        let expr = Expr {
                            kind: ExpKind::Unary(UnOp::Not, Box::new(_expr.clone())),
                            span: negation_token.span.to(&_expr.span),
                        };
                        _tl_stack.push(TranslatorStack::Expression(expr));
                    } else {
                        negation_token.span.to(&_expr.span);
                        _errors.push(ParseError {
                            token: negation_token,
                            message: String::from(_INVALID_NEGATION_EXPR_USE),
                            production_end: false,
                        });
                    }
                }
                ExpKind::Lit(lit) => {
                    if let Literal::Boolean(_) = lit {
                        let expr = Expr {
                            kind: ExpKind::Unary(UnOp::Not, Box::new(_expr.clone())),
                            span: negation_token.span.to(&_expr.span),
                        };
                        _tl_stack.push(TranslatorStack::Expression(expr));
                    } else {
                        negation_token.span.to(&_expr.span);
                        _errors.push(ParseError {
                            token: negation_token,
                            message: String::from(_INVALID_NEGATION_EXPR_USE),
                            production_end: false,
                        });
                    }
                }
            }
        }
    }
}
