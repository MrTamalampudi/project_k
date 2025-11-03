use crate::{
    ast::{
        arguments::Args,
        expression::{ExpKind, Expr, Literal},
        primitives::Primitives,
    },
    parser::{errors::EXPECT_STRING_EXPR, locator::LocatorStrategy},
    token::Token,
};

pub struct Shared;

impl Shared {
    pub fn get_locator_arg(expr: &Expr) -> Result<Args, String> {
        if expr.primitive != Primitives::String {
            return Err(EXPECT_STRING_EXPR.to_string());
        }
        if let ExpKind::Lit(Literal::String(locator)) = &expr.kind {
            Ok(Args::Locator(LocatorStrategy::parse(locator)))
        } else {
            Ok(Args::Expr(expr.clone()))
        }
    }
}
