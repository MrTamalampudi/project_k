use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expression::Expr,
        getter::Getter,
        primitives::Primitives,
        teststep::{GetMethod, Next, Teststep},
    },
    class::{Method, CUSTOM},
    location::{Span, SpanTrait},
};

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: VarRHS,
    pub method: Method,
    pub next: Option<Rc<RefCell<Teststep>>>,
    pub span: Span,
}

impl VarDecl {
    pub fn new(name: String, type_: Primitives, rhs: VarRHS, span: Span) -> VarDecl {
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

#[derive(Debug, Clone, PartialEq)]
pub enum VarRHS {
    Getter(Getter),
    Expression(Expr),
}

impl VarRHS {
    pub fn get_type(&self) -> Primitives {
        match self {
            VarRHS::Getter(getter) => getter.returns.clone(),
            VarRHS::Expression(expr) => expr.primitive.clone(),
        }
    }
}

impl GetMethod for VarDecl {
    fn get_method(&self) -> Method {
        self.method.clone()
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

impl SpanTrait for VarDecl {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl SpanTrait for VarRHS {
    fn get_span(&self) -> Span {
        match self {
            VarRHS::Expression(expr) => expr.span.clone(),
            VarRHS::Getter(getter) => getter.span.clone(),
        }
    }
}
