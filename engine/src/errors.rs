#![allow(dead_code)]
use ast::IdentifierValue;

pub type ExpressionEvalResult = Result<IdentifierValue, String>;

//***** Literal *****
pub(crate) const INT_OVERFLOW: &'static str = "Integer Overflowed";

//***** Literal *****
pub(crate) const EXPECT_LITERAL: &'static str = "Expected literal expression";
pub(crate) const INVALID_LOC_EXPR: &'static str = "Invalid locator expression";
pub(crate) const INVALID_INPUT: &'static str = "Invalid input";

//***** Binary *****
pub(crate) const INVALID_UNARY_OP: &'static str = "Invalid unary operation";

//***** Binary *****
pub(crate) const EXPECT_BINARY: &'static str = "Expected binary expression";
pub(crate) const INVALID_ADD_OP: &'static str = "Invalid addition operation";
pub(crate) const INVALID_SUB_OP: &'static str = "Invalid subtraction operation";
pub(crate) const INVALID_AND_OP: &'static str = "Invalid logical and operation";
pub(crate) const INVALID_OR_OP: &'static str = "Invalid logical or operation";
pub(crate) const INVALID_EQ_OP: &'static str = "Invalid equivalent operation";
pub(crate) const INVALID_CMP_OP: &'static str = "Invalid comparision operation";

//***** Array *****
pub(crate) const ARRAY_EVAL: &'static str = "Error while evaluating array";
