use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
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
    String(String),
    Var(String),
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
