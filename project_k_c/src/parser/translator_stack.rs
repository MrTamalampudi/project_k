use crate::{
    ast::{action::Action, expression::Expr, getter::Getter, var_decl::VarDecl},
    location::{Span, SpanTrait},
    parser::errors::EXPECT_EXPR,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    TestStep(Action),
    Getter(Getter),
    VarDecl(VarDecl),
    Expression(Expr),
}

impl TranslatorStack {
    pub fn get_expression(&self) -> Result<Expr, (String, Span)> {
        if let TranslatorStack::Expression(expr) = self {
            Ok(expr.clone())
        } else {
            Err((EXPECT_EXPR.to_string(), self.get_span()))
        }
    }
}

impl SpanTrait for TranslatorStack {
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

pub trait TLVec {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)>;
}

impl TLVec for Vec<TranslatorStack> {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)> {
        let tl = self.pop().unwrap();
        tl.get_expression()
    }
}
