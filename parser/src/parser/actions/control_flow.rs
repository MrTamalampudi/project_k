use std::ops::Range;

use crate::a_types;
use crate::keywords::NTokenType;
use crate::parser::errors::{EXPECT_ARRAY, EXPECT_BOOL_EXPR, EXPECT_VARIABLE};
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use ast::{
    expression::{ExpKind, Expr, Literal},
    ForLoop, IdentifierValue, IfStmt, Primitives, TestCase, Teststep,
};
use class::{ControlFlowAction, Method, CONTROL_FLOW};
use logos::Span;
use macros::{pop_body, pop_else, pop_expr, pop_token};
use manodae::error::ParseError;

pub struct ControlFlow;

impl ControlFlow {
    #[allow(non_snake_case)]
    #[pop_token(_in, ident)]
    #[pop_expr(iter)]
    pub fn HELPER(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<(NTokenType, Span)>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        if let ExpKind::Lit(Literal::Ident(name, _)) = &iter.kind {
            let value = _testcase.variables.get(name);
            let IdentifierValue::Array(_, _) = value.unwrap() else {
                _errors.push_error(&iter.span, EXPECT_ARRAY.to_string());
                return;
            };
        };
        let primitive = iter.primitive;
        if let NTokenType::IDENTIFIER(ident) = &ident.0 {
            _testcase
                .variables
                .insert(ident.clone(), primitive.to_identifier_value());
        }
        _token_stack.push(ident);
        _tl_stack.push_expr(iter);
    }
}

impl ControlFlowAction for ControlFlow {
    a_types!();

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _if_token)]
    #[pop_expr(cond_expr)]
    #[pop_body]
    #[pop_else]
    fn IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = _if_token.1.start.._r_curly_brace_token.1.start;
        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&cond_expr.span, EXPECT_BOOL_EXPR.to_string());
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
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = _else_token.1.start.._r_curly_brace_token.1.end;
        if cond_expr.primitive != Primitives::Boolean {
            _errors.push_error(&cond_expr.span, EXPECT_BOOL_EXPR.to_string());
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
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = _else_token.1.start.._r_curly_brace_token.1.end;
        let stmt = IfStmt {
            span,
            condition: Expr {
                kind: ExpKind::Lit(Literal::Boolean(true)),
                span: Range::default(),
                primitive: Primitives::Boolean,
            },
            body,
            or_else: Box::new(None),
            method: Method::CONTROL_FLOW(CONTROL_FLOW::ELSE),
        };
        _tl_stack.push(TranslatorStack::IfStmt(stmt));
    }

    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, _while_token)]
    #[pop_expr(condition)]
    #[pop_body]
    fn WHILE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) {
        let span = _while_token.1.start.._r_curly_brace_token.1.end;
        let stmt = IfStmt {
            span,
            condition,
            body,
            or_else: Box::new(None),
            method: Method::CONTROL_FLOW(CONTROL_FLOW::WHILE),
        };
        _tl_stack.push_step(Teststep::If(stmt));
    }

    //For Expression In Expression L_CurlyBrace Newline Teststeps R_CurlyBrace
    #[pop_token(_r_curly_brace_token, _l_curly_brace_token, target, for_token)]
    #[pop_expr(iter)]
    #[pop_body]
    fn FOR(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error>,
    ) {
        let span = for_token.1.start.._r_curly_brace_token.1.end;
        let target = if let NTokenType::IDENTIFIER(ident) = target.0 {
            ident.clone()
        } else {
            _errors.push_error(&target.1, EXPECT_VARIABLE.to_string());
            return;
        };

        let stmt = ForLoop {
            span,
            iter,
            target,
            body,
            method: Method::CONTROL_FLOW(CONTROL_FLOW::FOR),
        };
        _tl_stack.push_step(Teststep::For(stmt));
    }
}
