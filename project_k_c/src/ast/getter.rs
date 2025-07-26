use std::collections::HashMap;

use crate::{
    ast::{arguments::Args, primitives::Primitives, testcase_body::GetMethod},
    class::Method,
};

#[derive(Debug, Clone)]
pub struct Getter {
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}

impl GetMethod for Getter {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}
