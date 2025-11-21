use crate::{
    ast::{
        action::Action,
        expression::Expr,
        teststep::{Body, Teststep},
        var_decl::VarDecl,
    },
    parser::errors::EXPECT_EXPR,
};

use span::{Span, SpanData};
use span_macro::Span;

#[derive(Debug, Clone, PartialEq, Span)]
pub enum TranslatorStack {
    Body(Body),
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

pub trait TLVec {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)>;
    fn push_expr(&mut self, expr: Expr);
    fn push_step(&mut self, teststep: Teststep);
    fn get_body(&mut self) -> Body;
}

impl TLVec for Vec<TranslatorStack> {
    fn pop_expr(&mut self) -> Result<Expr, (String, Span)> {
        let tl = self.pop().unwrap();
        tl.get_expression()
    }
    fn push_expr(&mut self, expr: Expr) {
        self.push(TranslatorStack::Expression(expr));
    }
    fn push_step(&mut self, teststep: Teststep) {
        if let Some(TranslatorStack::Body(body)) = self.last_mut() {
            body.insert_teststep(teststep);
        };
    }
    fn get_body(&mut self) -> Body {
        match self.pop() {
            Some(TranslatorStack::Body(body)) => body,
            Some(_) => panic!("error"),
            None => panic!("error"),
        }
    }
}
