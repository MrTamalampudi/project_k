use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{getter::Getter, if_stmt::IfStmt, teststep::TestStep, var_decl::VarDecl},
    class::Method,
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TestcaseBody {
    TESTSTEP(TestStep),
    IF(IfStmt),
    GETTER(Getter),
    VAR_DECL(VarDecl),
}

pub trait GetMethod {
    fn get_method(&self) -> Method;
}

impl GetMethod for TestcaseBody {
    fn get_method(&self) -> Method {
        match self {
            TestcaseBody::GETTER(step) => step.get_method(),
            TestcaseBody::TESTSTEP(step) => step.get_method(),
            TestcaseBody::IF(step) => step.get_method(),
            TestcaseBody::VAR_DECL(step) => step.get_method(),
        }
    }
}

pub trait Next {
    fn set_next(&mut self, next: Rc<RefCell<TestcaseBody>>);
    fn get_next(&self) -> Option<Rc<RefCell<TestcaseBody>>>;
}
