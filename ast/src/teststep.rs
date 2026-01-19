use std::ops::Range;

use macros::Method;
use macros::Span;
use span::SpanData;

use crate::for_loop::ForLoop;
use crate::{action::Action, getter::Getter, if_stmt::IfStmt, var_decl::VarDecl};
use class::Method;

#[derive(PartialEq, Clone, Debug, Span)]
pub struct Body {
    pub span: Range<usize>,
    pub teststeps: Vec<Teststep>,
}

impl Body {
    pub fn new() -> Self {
        Body {
            span: 0..0,
            teststeps: vec![],
        }
    }
    pub fn insert_teststep(&mut self, teststep: Teststep) {
        let span = self.span_to(&teststep.get_span());
        if self.teststeps.is_empty() {
            self.span = teststep.get_span();
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
