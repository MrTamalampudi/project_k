use std::{cell::RefCell, rc::Rc};

use macros::Method;
use macros::Span;
use span::Location;
use span::Span;
use span::SpanData;

use crate::for_loop::ForLoop;
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

#[derive(Debug, Clone, PartialEq, Span, Method)]
#[allow(non_camel_case_types)]
pub enum Teststep {
    Action(Action),
    If(IfStmt),
    Getter(Getter),
    VarDecl(VarDecl),
    For(ForLoop),
}
