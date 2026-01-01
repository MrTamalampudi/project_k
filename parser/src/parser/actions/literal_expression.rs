use class::LiteralExpressionAction;
use macros::pop_token;
use manodae::error::ParseError;

use crate::{
    a_types,
    keywords::TokenType,
    parser::{
        errors::{EMPTY_ARRAY_EXPR, EXPECT_BOOL_EXPR, VARIABLE_NOT_DEFINED},
        errorss::ActionError,
        translator_stack::{TLVec, TranslatorStack},
    },
    token::Token,
};
use ast::{
    expression::{ExpKind, Expr, Literal as LE},
    primitives::Primitives,
    testcase::TestCase,
};

pub struct LiteralExpression {}

impl LiteralExpressionAction for LiteralExpression {
    a_types!();
    #[pop_token(string_token)]
    fn STRING(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        if let TokenType::STRING(string) = string_token.get_token_type() {
            let expr_kind = ExpKind::Lit(LE::String(string));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                primitive: Primitives::String,
                span: string_token.span,
                kind: expr_kind,
            }));
        }
    }

    #[pop_token(number_token)]
    fn NUMBER(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        if let TokenType::NUMBER(number) = number_token.get_token_type() {
            let expr_kind = ExpKind::Lit(LE::Number(number));
            _tl_stack.push(TranslatorStack::Expression(Expr {
                primitive: Primitives::Number,
                span: number_token.span,
                kind: expr_kind,
            }));
        }
    }

    #[pop_token(boolean_token)]
    fn BOOLEAN(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
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

    #[pop_token(ident_token)]
    fn IDENT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        if let TokenType::IDENTIFIER(ident) = ident_token.get_token_type() {
            let variable = _testcase.variables.get(&ident);
            if variable.is_none() {
                _errors.push(ParseError {
                    token: ident_token.clone(),
                    message: String::from(VARIABLE_NOT_DEFINED),
                    production_end: false,
                });
                return;
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

    #[pop_token(r_space_brac, l_space_brac)]
    fn ARRAY(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        let mut expr_arr = vec![];
        //pop expression from tl_stack till ArrayDelim
        while Some(&TranslatorStack::ArrayDelim) != _tl_stack.last() {
            let expr = match _tl_stack.pop_expr() {
                Some(expr) => expr,
                None => {
                    return;
                }
            };
            expr_arr.push(expr);
        }
        //remove ArrayDelim
        _tl_stack.pop();
        //because we push from backside so we need to reverse
        // [a,b,c] -> [c,b,a] -> [a,b,c]
        // ^orginal  ^expr_arr  ^reverse
        expr_arr.reverse();
        let span = l_space_brac.span.to(&r_space_brac.span);
        if expr_arr.is_empty() {
            //if it is an empty array it is impossible to determine
            //its primitive at the time of declaration
            _errors.push_error(&l_space_brac, &span, EMPTY_ARRAY_EXPR.to_string());
            return;
        }
        let primitive = expr_arr.first().unwrap().primitive;
        let kind = ExpKind::Array(expr_arr);
        _tl_stack.push(TranslatorStack::Expression(Expr {
            kind,
            span,
            primitive,
        }));
    }
}
