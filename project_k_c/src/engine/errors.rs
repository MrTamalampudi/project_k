use crate::ast::identifier_value::IdentifierValue;

pub type ExpressionEvalResult = Result<IdentifierValue, String>;

//***** Literal *****
pub const EXPECT_LITERAL: &'static str = "Expected literal expression";

//***** Binary *****
pub const EXPECT_BINARY: &'static str = "Expected binary expression";
pub const INVALID_ADD_OP: &'static str = "Invalid addition operation";
