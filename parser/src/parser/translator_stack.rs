use ast::{
    action::Action,
    expression::Expr,
    if_stmt::IfStmt,
    teststep::{Body, Teststep},
    var_decl::VarDecl,
};

use class::{Method, CONTROL_FLOW};

#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorStack {
    Body(Body),
    TestStep(Action),
    VarDecl(VarDecl),
    Expression(Expr),
    IfStmt(IfStmt),
    ArrayDelim,
}

impl TranslatorStack {
    pub fn get_expression(&self) -> Option<Expr> {
        if let TranslatorStack::Expression(expr) = self {
            Some(expr.clone())
        } else {
            None
        }
    }
}

pub trait TLVec {
    fn pop_expr(&mut self) -> Option<Expr>;
    fn push_expr(&mut self, expr: Expr);
    fn push_step(&mut self, teststep: Teststep);
    fn pop_body(&mut self) -> Body;
    fn pop_else(&mut self) -> Option<IfStmt>;
}

impl TLVec for Vec<TranslatorStack> {
    fn pop_expr(&mut self) -> Option<Expr> {
        let Some(TranslatorStack::Expression(_)) = self.last() else {
            return None;
        };
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
    fn pop_body(&mut self) -> Body {
        match self.pop() {
            Some(TranslatorStack::Body(body)) => body,
            Some(a) => panic!("some error {a:#?}"),
            None => panic!("errorddddddd"),
        }
    }
    fn pop_else(&mut self) -> Option<IfStmt> {
        if !matches!(
            self.last(),
            Some(TranslatorStack::IfStmt(stmt))
            if stmt.method != Method::CONTROL_FLOW(CONTROL_FLOW::IF)
        ) {
            return None;
        }
        if let TranslatorStack::IfStmt(stmt) = self.pop().unwrap() {
            return Some(stmt);
        } else {
            return None;
        }
    }
}
