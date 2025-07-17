use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{arguments::Args, testcase::TestcaseBody},
    class::{Class, Method},
    location::Location,
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestStep {
    start: Location,
    end: Location,
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
