use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        arguments::Args,
        teststep::{GetMethod, Next, Teststep},
    },
    class::{Class, Method},
    location::{Span, SpanTrait},
};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Action {
    pub span: Span,
    pub class: Class,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub next: Option<Rc<RefCell<Teststep>>>,
}

impl Action {
    pub fn new(
        span: Span,
        class: Class,
        method: Method,
        arguments: HashMap<&'static str, Args>,
    ) -> Action {
        Action {
            span,
            class,
            method,
            arguments,
            next: None,
        }
    }
}

impl GetMethod for Action {
    fn get_method(&self) -> Method {
        self.method.clone()
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

impl SpanTrait for Action {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}
