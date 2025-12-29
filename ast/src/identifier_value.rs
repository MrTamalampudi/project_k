use crate::{locator::LocatorStrategy, primitives::Primitives};
use std::mem::discriminant;
use thirtyfour::WebElement;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentifierValue {
    String(Option<String>),
    Number(Option<isize>),
    Element(Option<WebElement>),
    Boolean(Option<bool>),
}

impl IdentifierValue {
    pub fn to_primitive(&self) -> Primitives {
        match self {
            IdentifierValue::String(_) => Primitives::String,
            IdentifierValue::Number(_) => Primitives::Number,
            IdentifierValue::Element(_) => Primitives::Element,
            IdentifierValue::Boolean(_) => Primitives::Boolean,
        }
    }

    pub fn matches(&self, value: &IdentifierValue) -> bool {
        discriminant(self) == discriminant(value)
    }
}
