use crate::{
    ast::{action::Action, expression::Expr, var_decl::VarDecl},
    parser::errors::EXPECT_EXPR,
};

use span::{Span, SpanTrait};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    TestStep(Action),
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
            Expression(expr) => expr.get_span(),
            VarDecl(var) => var.get_span(),
        }
    }
}

pub trait TLVec {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)>;
    fn push_expr(&mut self, expr: Expr);
}

impl TLVec for Vec<TranslatorStack> {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)> {
        let tl = self.pop().unwrap();
        tl.get_expression()
    }
    fn push_expr(&mut self, expr: Expr) {
        self.push(TranslatorStack::Expression(expr));
    }
}
