use std::{cell::RefCell, rc::Rc};

use macros::Span;
use span::Location;
use span::Span;
use span::SpanData;

use crate::{action::Action, getter::Getter, if_stmt::IfStmt, var_decl::VarDecl};
use class::Method;

#[derive(PartialEq, Clone, Debug, Span)]
pub struct Body {
    pub span: Span,
    pub teststeps: Vec<Teststep>,
}

impl Body {
    pub fn new() -> Self {
        Body {
            span: Span {
                start: Location::dummy(),
                end: Location::dummy(),
            },
            teststeps: vec![],
        }
    }
    pub fn insert_teststep(&mut self, teststep: Teststep) {
        let body_span = self.get_span();
        let teststep_span = teststep.get_span();
        let span = body_span.to(&teststep_span);
        if self.teststeps.is_empty() {
            self.span = teststep_span;
        } else {
            self.span = span
        }
        self.teststeps.push(teststep);
    }
}

#[derive(Debug, Clone, PartialEq, Span)]
#[allow(non_camel_case_types)]
pub enum Teststep {
    Action(Action),
    If(IfStmt),
    Getter(Getter),
    VarDecl(VarDecl),
}

pub trait GetMethod {
    fn get_method(&self) -> Method;
}

impl GetMethod for Teststep {
    fn get_method(&self) -> Method {
        match self {
            Teststep::Getter(step) => step.get_method(),
            Teststep::Action(step) => step.get_method(),
            Teststep::If(step) => step.get_method(),
            Teststep::VarDecl(step) => step.get_method(),
        }
    }
}

pub trait Next {
    fn set_next(&mut self, next: Rc<RefCell<Teststep>>);
    fn get_next(&self) -> Option<Rc<RefCell<Teststep>>>;
}
