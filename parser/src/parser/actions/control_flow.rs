use crate::a_types;
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errorss::ActionError;
use crate::{
    parser::translator_stack::{TLVec, TranslatorStack},
    token::Token,
};
use ast::expression::ExpKind;
use ast::teststep::Teststep;
use ast::{
    expression::{Expr, Literal},
    if_stmt::IfStmt,
    primitives::Primitives,
    testcase::TestCase,
};
use class::{ControlFlowAction, Method, CONTROL_FLOW};
use macros::{pop_body, pop_else, pop_expr, pop_token};
use manodae::error::ParseError;
use span::Span;

pub struct ControlFlow;

impl ControlFlowAction for ControlFlow {
    a_types!();

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _if_token)]
    #[pop_expr(cond_expr)]
    #[pop_body]
    #[pop_else]
    fn IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = _if_token.span.to(&_r_curly_brace_token.span);
        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&_if_token, &cond_expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }
        let stmt = IfStmt {
            span,
            condition: cond_expr,
            body,
            or_else,
            method: Method::CONTROL_FLOW(CONTROL_FLOW::IF),
        };
        _tl_stack.push_step(Teststep::If(stmt));
    }

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _if_token, _else_token)]
    #[pop_expr(cond_expr)]
    #[pop_body]
    #[pop_else]
    fn ELSE_IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let span = _else_token.span.to(&_r_curly_brace_token.span);
        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&_else_token, &cond_expr.span, EXPECT_BOOL_EXPR.to_string());
            return;
        }

        let stmt = IfStmt {
            span,
            condition: cond_expr,
            body,
            or_else,
            method: Method::CONTROL_FLOW(CONTROL_FLOW::ELSE_IF),
        };
        _tl_stack.push(TranslatorStack::IfStmt(stmt));
    }

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _else_token)]
    #[pop_body]
    fn ELSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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
            method: Method::CONTROL_FLOW(CONTROL_FLOW::ELSE),
        };
        _tl_stack.push(TranslatorStack::IfStmt(stmt));
    }
}
