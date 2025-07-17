use crate::parser::locator::LocatorStrategy;

#[derive(Debug, Clone)]
pub enum Args {
    String(String),
    Locator(LocatorStrategy),
}
