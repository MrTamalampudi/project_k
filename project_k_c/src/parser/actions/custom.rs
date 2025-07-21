use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase::TestcaseBody;
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
        testcase: &mut TestCase,
        token_stack: &mut Vec<Token>,
        tl_stack: &mut Vec<TranslatorStack>,
        errors: &mut Vec<ParseError<Token>>,
    ) {
        let var_rhs = match tl_stack.last() {
            Some(rhs) => rhs,
            None => {
                //check if it is a identifier
                let rhs_ident = token_stack.get(2);
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

        let identifier = match token_stack.first() {
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
            TranslatorStack::Getter(getter) => VarDecl {
                name: identifier.clone(),
                type_: getter.returns.clone(),
                rhs: VarRHS::Getter(getter.clone()),
            },
            TranslatorStack::String(string) => VarDecl {
                name: identifier.clone(),
                type_: Primitives::String,
                rhs: VarRHS::String(string.clone()),
            },
            TranslatorStack::Ident(ident) => {
                let variable = testcase.variables.get(ident);
                if let Some(variable_) = variable {
                    VarDecl {
                        name: identifier.clone(),
                        type_: variable_.to_primitive(),
                        rhs: VarRHS::Var(ident.clone()),
                    }
                } else {
                    if let Some(token) = token_stack.get(2) {
                        errors.push(ParseError {
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

        if let Some(variable) = testcase.variables.get(identifier) {
            if variable.to_primitive().ne(&var_decl.type_) {
                if let Some(token) = token_stack.get(2) {
                    errors.push(ParseError {
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

        testcase.insert_teststep(TestcaseBody::VarDecl(var_decl.clone()));
        testcase.insert_variable(var_decl.clone());
        // println!("variables {:#?}", testcase.variables);
        tl_stack.clear();
        token_stack.clear();
    }
}
