use std::{cell::RefCell, rc::Rc};

use crate::{
    expression::Expr,
    primitives::Primitives,
    teststep::{Next, Teststep},
};
use class::{CUSTOM, Method};
use macros::{Method, Span};
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: Expr,
    pub method: Method,
    pub next: Option<Rc<RefCell<Teststep>>>,
    pub span: Span,
}

impl VarDecl {
    pub fn new(name: String, type_: Primitives, rhs: Expr, span: Span) -> VarDecl {
        VarDecl {
            name,
            type_,
            rhs,
            method: Method::CUSTOM(CUSTOM::VAR_DECLARATION),
            next: None,
            span,
        }
    }
}

impl Next for VarDecl {
    fn set_next(&mut self, next: Rc<RefCell<Teststep>>) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Option<Rc<RefCell<Teststep>>> {
        self.next.clone()
    }
}
