use crate::{
    ast::{action::Action, expression::Expr, getter::Getter, var_decl::VarDecl},
    location::{Span, Span_Trait},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    TestStep(Action),
    Getter(Getter),
    VarDecl(VarDecl),
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
