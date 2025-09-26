use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        arguments::Args,
        testcase_body::{GetMethod, Next, TestcaseBody},
    },
    class::{Class, Method},
    location::{Location, Span, Span_Trait},
};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct TestStep {
    pub span: Span,
    pub class: Class,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub next: Option<Rc<RefCell<TestcaseBody>>>,
}

impl TestStep {
    pub fn new(
        span: Span,
        class: Class,
        method: Method,
        arguments: HashMap<&'static str, Args>,
    ) -> TestStep {
        TestStep {
            span,
            class,
            method,
            arguments,
            next: None,
        }
    }
}

impl GetMethod for TestStep {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}

impl Next for TestStep {
    fn set_next(&mut self, next: Rc<RefCell<TestcaseBody>>) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Option<Rc<RefCell<TestcaseBody>>> {
        self.next.clone()
    }
}

impl Span_Trait for TestStep {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}
