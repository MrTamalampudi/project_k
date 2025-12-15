use crate::ast::expression::ExpKind;
use crate::ast::teststep::Teststep;
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errorss::ActionError;
use crate::types;
use crate::{
    ast::{
        expression::{Expr, Literal},
        if_stmt::IfStmt,
        primitives::Primitives,
        testcase::TestCase,
    },
    parser::translator_stack::{TLVec, TranslatorStack},
    token::Token,
};
use class::{ConditionalStmtAction, Method, CONDITIONAL_STMT};
use macros::pop_token;
use manodae::error::ParseError;
use span::Span;

pub struct Conditional;

impl ConditionalStmtAction for Conditional {
    types!();
    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _if_token)]
    fn IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = _if_token.span.to(&_r_curly_brace_token.span);
        let or_else = Box::new(_tl_stack.pop_else());
        let body = _tl_stack.pop_body();
        let cond_expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((e, span)) => {
                _errors.push_error(&_if_token, &span, e);
                return;
            }
        };
        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&_if_token, &cond_expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }
        let stmt = IfStmt {
            span,
            condition: cond_expr,
            body,
            or_else,
            method: Method::CONDITIONAL_STMT(CONDITIONAL_STMT::IF),
        };
        _tl_stack.push_step(Teststep::If(stmt));
    }

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _if_token, _else_token)]
    fn ELSE_IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = _else_token.span.to(&_r_curly_brace_token.span);
        let or_else = Box::new(_tl_stack.pop_else());
        let body = _tl_stack.pop_body();
        let cond_expr = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((e, span)) => {
                _errors.push_error(&_else_token, &span, e);
                return;
            }
        };

        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&_else_token, &cond_expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }

        let stmt = IfStmt {
            span,
            condition: cond_expr,
            body,
            or_else,
            method: Method::CONDITIONAL_STMT(CONDITIONAL_STMT::ELSE_IF),
        };
        _tl_stack.push(TranslatorStack::IfStmt(stmt));
    }

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _else_token)]
    fn ELSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let body = _tl_stack.pop_body();
        let span = _else_token.span.to(&_r_curly_brace_token.span);
        let stmt = IfStmt {
            span,
            condition: Expr {
                kind: ExpKind::Lit(Literal::Boolean(true)),
                span: Span::dummy(),
                primitive: Primitives::Boolean,
            },
            body,
            or_else: Box::new(None),
            method: Method::CONDITIONAL_STMT(CONDITIONAL_STMT::ELSE),
        };
        _tl_stack.push(TranslatorStack::IfStmt(stmt));
    }
}
