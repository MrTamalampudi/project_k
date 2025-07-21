use std::collections::HashMap;

use crate::{
    ast::{arguments::Args, primitives::Primitives},
    class::Method,
};

#[derive(Debug, Clone)]
pub struct Getter {
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}
