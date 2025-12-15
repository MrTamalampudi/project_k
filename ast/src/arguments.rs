use crate::{expression::Expr, locator::LocatorStrategy};

pub const LOCATOR_ARGKEY: &'static str = "locator";
pub const URL_ARGKEY: &'static str = "url";
pub const ATTRIBUTE_ARGKEY: &'static str = "attribute";
pub const SECS_ARGKEY: &'static str = "seconds";
pub const EXPR_ARGKEY: &'static str = "expression";

#[derive(Debug, Clone, PartialEq)]
pub enum Args {
    Expr(Expr),
    String(String),
    Locator(LocatorStrategy),
    Ident(String),
    Number(isize),
    None,
}
