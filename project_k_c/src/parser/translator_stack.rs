use crate::{
    ast::{expression::Expr, getter::Getter, teststep::TestStep, var_decl::VarDecl},
    location::{Span, Span_Trait},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    TestStep(TestStep),
    Getter(Getter),
    VarDecl(VarDecl),
    String(String),
    Ident(String),
    Expression(Expr),
}

impl Span_Trait for TranslatorStack {
    fn get_span(&self) -> Span {
        use TranslatorStack::*;
        match self {
            TestStep(teststep) => teststep.get_span(),
            Getter(getter) => getter.get_span(),
            Expression(expr) => expr.get_span(),
            VarDecl(var) => var.get_span(),
        }
    }
}
