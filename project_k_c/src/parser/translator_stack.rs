use crate::ast::{expression::Expr, getter::Getter, teststep::TestStep, var_decl::VarDecl};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    TestStep(TestStep),
    Getter(Getter),
    VarDecl(VarDecl),
    String(String),
    Ident(String),
    Expression(Expr),
}
