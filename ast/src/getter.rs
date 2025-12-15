use std::collections::HashMap;

use crate::{arguments::Args, primitives::Primitives, teststep::GetMethod};
use class::Method;
use macros::Span;
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq, Span)]
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
