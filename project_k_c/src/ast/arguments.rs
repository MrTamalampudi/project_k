use crate::parser::locator::LocatorStrategy;

pub const LOCATOR_ARGKEY: &'static str = "locator";
pub const URL_ARGKEY: &'static str = "url";

#[derive(Debug, Clone)]
pub enum Args {
    String(String),
    Locator(LocatorStrategy),
}
