use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    arguments::Args,
    teststep::{Next, Teststep},
};
use class::Method;
use macros::{Method, Span};
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq, Span, Method)]
#[allow(unused)]
pub struct Action {
    pub span: Span,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub next: Option<Rc<RefCell<Teststep>>>,
}

impl Action {
    pub fn new(span: Span, method: Method, arguments: HashMap<&'static str, Args>) -> Action {
        Action {
            span,
            method,
            arguments,
            next: None,
        }
    }
}

impl Next for Action {
    fn set_next(&mut self, next: Rc<RefCell<Teststep>>) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Option<Rc<RefCell<Teststep>>> {
        self.next.clone()
    }
}
