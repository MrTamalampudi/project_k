use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expression::Expr,
        getter::Getter,
        primitives::Primitives,
        testcase_body::{GetMethod, Next, TestcaseBody},
    },
    class::{Method, CUSTOM},
    location::{Span, Span_Trait},
};

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: VarRHS,
    pub method: Method,
    pub next: Option<Rc<RefCell<TestcaseBody>>>,
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
    fn set_next(&mut self, next: Rc<RefCell<TestcaseBody>>) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Option<Rc<RefCell<TestcaseBody>>> {
        self.next.clone()
    }
}

impl Span_Trait for VarDecl {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl Span_Trait for VarRHS {
    fn get_span(&self) -> Span {
        match self {
            VarRHS::Expression(expr) => expr.span.clone(),
            VarRHS::Getter(getter) => getter.span.clone(),
        }
    }
}
