use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{action::Action, getter::Getter, if_stmt::IfStmt, var_decl::VarDecl},
    class::Method,
};

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Teststep {
    Action(Action),
    IF(IfStmt),
    GETTER(Getter),
    VAR_DECL(VarDecl),
}

pub trait GetMethod {
    fn get_method(&self) -> Method;
}

impl GetMethod for Teststep {
    fn get_method(&self) -> Method {
        match self {
            Teststep::GETTER(step) => step.get_method(),
            Teststep::Action(step) => step.get_method(),
            Teststep::IF(step) => step.get_method(),
            Teststep::VAR_DECL(step) => step.get_method(),
        }
    }
}

pub trait Next {
    fn set_next(&mut self, next: Rc<RefCell<Teststep>>);
    fn get_next(&self) -> Option<Rc<RefCell<Teststep>>>;
}
