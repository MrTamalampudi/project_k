use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::Args;
use crate::ast::arguments::EXPR_ARGKEY;
use crate::ast::expression::ExpKind;
use crate::ast::expression::Literal;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::ast::var_decl::VarDecl;
use crate::ast::var_decl::VarRHS;
use crate::class::CustomAction;
use crate::class::CUSTOM;
use crate::keywords::TokenType;
use crate::location::Span;
use crate::location::Span_Trait;
use crate::parser::errors::EXPECT_BOOL_EXPR;
use crate::parser::errors::EXPECT_EXPR_OR_GETTER;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use manodae::error::ParseError;

pub struct Custom {}

impl CustomAction for Custom {
    //var = var_rhs
    //fetch var_rhs from tl_stack last element;
    fn VAR_DECLARATION(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        _token_stack.pop(); //pops assign token
        let identifier_token = _token_stack.pop().unwrap();
        let identifier = match identifier_token.get_token_type() {
            TokenType::STRING(ident) => ident,
            _ => return,
        };
        let var_rhs_tl = _tl_stack.pop().unwrap();
        let var_rhs = match var_rhs_tl {
            TranslatorStack::Expression(expr) => VarRHS::Expression(expr),
            TranslatorStack::Getter(getter) => VarRHS::Getter(getter),
            _ => {
                let mut i_token = identifier_token.clone();
                i_token.span = var_rhs_tl.get_span();
                _errors.push(ParseError {
                    token: i_token,
                    message: EXPECT_EXPR_OR_GETTER.to_string(),
                    production_end: false,
                });
                return;
            }
        };

        let span = identifier_token.span.to(&var_rhs.get_span());
        let var_decl = VarDecl::new(identifier.clone(), var_rhs.get_type(), var_rhs, span);

        if let Some(variable) = _testcase.variables.get(&identifier) {
            if variable.to_primitive().ne(&var_decl.type_) {
                if let Some(token) = _token_stack.get(2) {
                    _errors.push(ParseError {
                        token: token.clone(),
                        message: String::from(format!(
                            "{}, expected {} found {}",
                            MISMATCHED_TYPES,
                            variable.to_primitive().to_string(),
                            var_decl.type_.to_string()
                        )),
                        production_end: false,
                    });
                };
                return;
            }
        }

        _testcase.insert_teststep(Teststep::VarDecl(var_decl.clone()));
        _testcase.insert_variable(var_decl);
        _tl_stack.clear();
        _token_stack.clear();
    }

    fn ASSERT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        let assert_token = _token_stack.pop().unwrap();
        let tl_expr = _tl_stack.pop().unwrap();
        let expression = match &tl_expr {
            TranslatorStack::Expression(expr) => expr.clone(),
            _ => {
                let mut at_token = assert_token.clone();
                at_token.span = tl_expr.get_span();
                _errors.push(ParseError {
                    token: at_token,
                    message: EXPECT_BOOL_EXPR.to_string(),
                    production_end: false,
                });
                return;
            }
        };

        let is_boolean_expr = match &expression.kind {
            ExpKind::Binary(op, _, _) => op.is_bool_op(),
            ExpKind::Unary(_, _) => true,
            ExpKind::Lit(lit) => match lit {
                Literal::Boolean(_) => true,
                Literal::Ident(_, primitive) => {
                    if let Primitives::Boolean = primitive {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        };

        if !is_boolean_expr {
            let mut at_token = assert_token.clone();
            at_token.span = tl_expr.get_span();
            _errors.push(ParseError {
                token: at_token,
                message: EXPECT_BOOL_EXPR.to_string(),
                production_end: false,
            });
            return;
        } else {
            let teststep = Action::new(
                Span {
                    start: assert_token.get_start_location(),
                    end: expression.span.end,
                },
                crate::class::Class::CUSTOM,
                crate::class::Method::CUSTOM(CUSTOM::ASSERT),
                HashMap::from([(EXPR_ARGKEY, Args::Expr(expression))]),
            );
            _testcase.insert_teststep(Teststep::Action(teststep));
            _token_stack.clear();
        }
    }
}
