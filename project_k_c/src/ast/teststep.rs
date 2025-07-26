use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        arguments::Args,
        testcase_body::{GetMethod, Next, TestcaseBody},
    },
    class::{Class, Method},
    location::Location,
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestStep {
    pub start: Location,
    pub end: Location,
    pub class: Class,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub next: Option<Rc<RefCell<TestcaseBody>>>,
}

impl TestStep {
    pub fn new(
        start: Location,
        end: Location,
        class: Class,
        method: Method,
        arguments: HashMap<&'static str, Args>,
    ) -> TestStep {
        TestStep {
            start,
            end,
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
