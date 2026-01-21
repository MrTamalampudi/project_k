#![allow(dead_code)]

pub(crate) const VALID_URL: &'static str = "Please provide a valid URL";
pub(crate) const VALID_URL_SHCEME: &'static str = "Please provide url only with scheme HTTPS";
pub(crate) const NEGATIVE_TIME: &'static str = "Time should be positive";

// ***** variable *****
pub(crate) const VARIABLE_NOT_DEFINED: &'static str = "Variable not yet defined";
pub(crate) const MISMATCHED_TYPES: &'static str = "Mismatched Types";
pub(crate) const EXPECT_EXPR_OR_GETTER: &'static str = "Expected expression or getter action";
pub(crate) const EXPECT_VARIABLE: &'static str = "Expected variable";

// ***** token *****
pub(crate) const _INVALID_TOKEN: &'static str = "Invalid token. Please check syntax";

// ***** expression *****
pub(crate) const EXPECT_BOOL_EXPR: &'static str = "Expected bool expression";
pub(crate) const EXPECT_EXPR: &'static str = "Expected expression";
pub(crate) const EXPECT_STRING_EXPR: &'static str = "Expected string expression";
pub(crate) const EXPECT_NUMBER_EXPR: &'static str = "Expected number expression";
pub(crate) const EMPTY_ARRAY_EXPR: &'static str = "Array shouldn't be empty";
pub(crate) const EXPECT_ARRAY: &'static str = "Expected array expression";
pub(crate) const _INVALID_NEGATION_EXPR_USE: &'static str =
    "Use Negation operator only on boolean operations";
pub(crate) const ASSERT_SUPPORTS_ONLY_BOOL: &'static str =
    "Assertion action supports only boolean expression";
