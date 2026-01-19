use std::{cell::RefCell, collections::HashMap, ops::Range, rc::Rc};

use crate::{arguments::Args, teststep::Teststep};
use class::Method;
use macros::{Method, Span};
use span::SpanData;

#[derive(Debug, Clone, PartialEq, Span, Method)]
#[allow(unused)]
pub struct Action {
    pub span: Range<usize>,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub next: Option<Rc<RefCell<Teststep>>>,
}

impl Action {
    pub fn new(
        span: Range<usize>,
        method: Method,
        arguments: HashMap<&'static str, Args>,
    ) -> Action {
        Action {
            span,
            method,
            arguments,
            next: None,
        }
    }
}
