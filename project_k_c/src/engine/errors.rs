use crate::ast::identifier_value::IdentifierValue;

pub type ExpressionEvalResult = Result<IdentifierValue, String>;

//***** Literal *****
pub const INT_OVERFLOW: &'static str = "Integer Overflowed";

//***** Literal *****
pub const EXPECT_LITERAL: &'static str = "Expected literal expression";

//***** Binary *****
pub const INVALID_UNARY_OP: &'static str = "Invalid unary operation";

//***** Binary *****
pub const EXPECT_BINARY: &'static str = "Expected binary expression";
pub const INVALID_ADD_OP: &'static str = "Invalid addition operation";
pub const INVALID_SUB_OP: &'static str = "Invalid subtraction operation";
pub const INVALID_AND_OP: &'static str = "Invalid logical and operation";
pub const INVALID_OR_OP: &'static str = "Invalid logical or operation";
pub const INVALID_EQ_OP: &'static str = "Invalid equivalent operation";
pub const INVALID_CMP_OP: &'static str = "Invalid comparision operation";
