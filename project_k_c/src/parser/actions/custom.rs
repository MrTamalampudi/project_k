use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::var_decl::VarDecl;
use crate::ast::var_decl::VarRHS;
use crate::class::CustomAction;
use crate::keywords::TokenType;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errors::VARIABLE_NOT_DEFINED;
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use slr_parser::error::ParseError;

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
        let var_rhs = match _tl_stack.last() {
            Some(rhs) => rhs,
            None => {
                //check if it is a identifier
                let rhs_ident = _token_stack.get(2);
                if let Some(ident) = rhs_ident {
                    match &ident.token_type {
                        TokenType::IDENTIFIER(ident_) => &TranslatorStack::Ident(ident_.clone()),
                        TokenType::STRING(string_) => &TranslatorStack::String(string_.clone()),
                        _ => return,
                    }
                } else {
                    return;
                }
            }
        };

        let identifier = match _token_stack.first() {
            Some(token) => match &token.token_type {
                TokenType::IDENTIFIER(ident) => ident,
                _ => {
                    return;
                }
            },
            None => {
                return;
            }
        };

        let var_decl = match var_rhs {
            TranslatorStack::Getter(getter) => VarDecl::new(
                identifier.clone(),
                getter.returns.clone(),
                VarRHS::Getter(getter.clone()),
            ),
            TranslatorStack::String(string) => VarDecl::new(
                identifier.clone(),
                Primitives::String,
                VarRHS::String(string.clone()),
            ),
            TranslatorStack::Ident(ident) => {
                let variable = _testcase.variables.get(ident);
                if let Some(variable_) = variable {
                    VarDecl::new(
                        identifier.clone(),
                        variable_.to_primitive(),
                        VarRHS::Var(ident.clone()),
                    )
                } else {
                    if let Some(token) = _token_stack.get(2) {
                        _errors.push(ParseError {
                            token: token.clone(),
                            message: String::from(VARIABLE_NOT_DEFINED),
                            production_end: false,
                        });
                    };

                    return;
                }
            }
            _ => {
                return;
            }
        };

        if let Some(variable) = _testcase.variables.get(identifier) {
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

        _testcase.insert_teststep(TestcaseBody::VAR_DECL(var_decl.clone()));
        _testcase.insert_variable(var_decl.clone());
        _tl_stack.clear();
        _token_stack.clear();
    }
}
