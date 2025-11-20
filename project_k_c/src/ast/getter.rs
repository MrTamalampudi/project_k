use std::collections::HashMap;

use crate::{
    ast::{arguments::Args, primitives::Primitives, teststep::GetMethod},
    class::Method,
};
use span::{Span, SpanTrait};

#[derive(Debug, Clone, PartialEq)]
pub struct Getter {
    pub span: Span,
    pub method: Method,
    pub arguments: HashMap<&'static str, Args>,
    pub returns: Primitives,
}

impl GetMethod for Getter {
    fn get_method(&self) -> Method {
        self.method.clone()
    }
}

impl SpanTrait for Getter {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}
