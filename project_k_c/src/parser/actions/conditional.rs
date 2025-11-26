use crate::ast::expression::ExpKind;
use crate::ast::teststep::Teststep;
use crate::class::{Method, CONDITIONAL_STMT};
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errorss::ActionError;
use crate::{
    ast::{
        expression::{Expr, Literal},
        if_stmt::IfStmt,
        primitives::Primitives,
        testcase::TestCase,
    },
    class::ConditionalStmtAction,
    parser::translator_stack::{TLVec, TranslatorStack},
    token::Token,
};
use manodae::error::ParseError;
use span::Span;

pub struct Conditional;

impl ConditionalStmtAction for Conditional {
    fn IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let _r_curly_brace_token = _token_stack.pop().unwrap();
        let _l_curly_brace_token = _token_stack.pop().unwrap();
        let _if_token = _token_stack.pop().unwrap();
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

    fn ELSE_IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let _r_curly_brace_token = _token_stack.pop().unwrap();
        let _l_curly_brace_token = _token_stack.pop().unwrap();
        let _if_token = _token_stack.pop().unwrap();
        let _else_token = _token_stack.pop().unwrap();
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

    fn ELSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let _r_curly_brace_token = _token_stack.pop().unwrap();
        let _l_curly_brace_token = _token_stack.pop().unwrap();
        let _else_token = _token_stack.pop().unwrap();
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
