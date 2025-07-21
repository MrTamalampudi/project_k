use thirtyfour::WebElement;

use crate::{ast::primitives::Primitives, parser::locator::LocatorStrategy};

#[derive(Debug, Clone)]
pub enum IdentifierValue {
    String(Option<String>),
    Number(Option<isize>),
    Locators(Option<LocatorStrategy>),
    Element(Option<WebElement>),
}

impl IdentifierValue {
    pub fn to_primitive(&self) -> Primitives {
        match self {
            IdentifierValue::String(_) => Primitives::String,
            IdentifierValue::Number(_) => Primitives::Number,
            IdentifierValue::Locators(_) => Primitives::Locators,
            IdentifierValue::Element(_) => Primitives::Element,
        }
    }
}
