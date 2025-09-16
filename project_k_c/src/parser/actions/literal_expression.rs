use manodae::error::ParseError;

use crate::{
    ast::{
        expression::{ExpKind, Expr, Literal as LE},
        testcase::TestCase,
    },
    class::LiteralExpressionAction,
    keywords::TokenType,
    parser::{errors::VARIABLE_NOT_DEFINED, translator_stack::TranslatorStack},
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
        if let TokenType::STRING(bool) = boolean_token.get_token_type() {
            if !(bool.as_str() == "true" || bool.as_str() == "false") {
                return;
            }
            let truth_value = matches!(bool.as_str(), "true" | "1");
            let expr_kind = ExpKind::Lit(LE::Boolean(truth_value));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                span: boolean_token.span,
                kind: expr_kind,
            }));
        }
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
            let expr_kind = ExpKind::Lit(LE::Ident(ident, variable_type));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                span: ident_token.span,
                kind: expr_kind,
            }));
        }
    }
}
