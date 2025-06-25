use crate::{
    ast::arguments::Args,
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
    pub arguments: Vec<Args>,
}

impl TestStep {
    pub fn new(
        start: Location,
        end: Location,
        class: Class,
        method: Method,
        arguments: Vec<Args>,
    ) -> TestStep {
        TestStep {
            start,
            end,
            class,
            method,
            arguments,
        }
    }
}
