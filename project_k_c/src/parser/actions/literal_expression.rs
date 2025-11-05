use manodae::error::ParseError;

use crate::{
    ast::{
        expression::{ExpKind, Expr, Literal as LE},
        primitives::Primitives,
        testcase::TestCase,
    },
    class::LiteralExpressionAction,
    keywords::TokenType,
    parser::{
        errors::{EXPECT_BOOL_EXPR, VARIABLE_NOT_DEFINED},
        errorss::ActionError,
        translator_stack::TranslatorStack,
    },
    token::Token,
};

pub struct LiteralExpression {}

impl LiteralExpressionAction for LiteralExpression {
    fn STRING(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let string_token = _token_stack.pop().unwrap();
        if let TokenType::STRING(string) = string_token.get_token_type() {
            let expr_kind = ExpKind::Lit(LE::String(string));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                primitive: Primitives::String,
                span: string_token.span,
                kind: expr_kind,
            }));
        }
    }
    fn NUMBER(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let number_token = _token_stack.pop().unwrap();
        if let TokenType::NUMBER(number) = number_token.get_token_type() {
            let expr_kind = ExpKind::Lit(LE::Number(number));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                primitive: Primitives::Number,
                span: number_token.span,
                kind: expr_kind,
            }));
        }
    }
    fn BOOLEAN(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let boolean_token = _token_stack.pop().unwrap();
        let truth_value = match boolean_token.token_type {
            TokenType::TRUE => true,
            TokenType::FALSE => false,
            _ => {
                _errors.push_error(
                    &boolean_token,
                    &boolean_token.span,
                    EXPECT_BOOL_EXPR.to_string(),
                );
                return;
            }
        };
        let expr_kind = ExpKind::Lit(LE::Boolean(truth_value));
        _tl_stack.push(TranslatorStack::Expression(Expr {
            primitive: Primitives::Boolean,
            span: boolean_token.span,
            kind: expr_kind,
        }));
    }
    fn IDENT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let ident_token = _token_stack.pop().unwrap();
        if let TokenType::IDENTIFIER(ident) = ident_token.get_token_type() {
            let variable = _testcase.variables.get(&ident);
            if variable.is_none() {
                _errors.push(ParseError {
                    token: ident_token.clone(),
                    message: String::from(VARIABLE_NOT_DEFINED),
                    production_end: false,
                });
            }
            let variable_type = variable.unwrap().to_primitive();
            let expr_kind = ExpKind::Lit(LE::Ident(ident, variable_type.clone()));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                primitive: variable_type,
                span: ident_token.span,
                kind: expr_kind,
            }));
        }
    }
}
