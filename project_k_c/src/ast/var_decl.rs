use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    getter::Getter, primitives::Primitives, testcase::TestcaseBody, teststep::TestStep,
};

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: VarRHS,
    pub next: Option<Rc<RefCell<TestcaseBody>>>,
}

impl VarDecl {
    pub fn new(name: String, type_: Primitives, rhs: VarRHS) -> VarDecl {
        VarDecl {
            name,
            type_,
            rhs,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum VarRHS {
    Getter(Getter),
    Teststep(TestStep),
    String(String),
    Var(String),
}
