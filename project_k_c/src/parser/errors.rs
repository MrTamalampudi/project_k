use crate::keywords::TokenType;
use core::fmt::{self, Formatter, Write};

macro_rules! parser_error {
    ($($name: ident = $description: expr,)+) => {
        /// Errors that can occur during parsing.
        ///
        /// This may be extended in the future so exhaustive matching is
        /// discouraged with an unused variant.
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        #[non_exhaustive]
        pub enum ParserError {
            $(
                $name,
            )+
        }

        impl fmt::Display for ParserError {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                        ParserError::$name => fmt.write_str($description),
                    )+
                }
            }
        }
    }
}

impl std::error::Error for ParserError {}

parser_error! {
    URL = "Please provide a valid URL",
    URL_HTTPS = "Please provide a valid HTTPS URL",
    CAPABILITY_KEY = "Please provide a valid capability key",
    ASSIGN_OP = "Please provide a valid assignment operator",
}

pub const VALID_URL: &'static str = "Please provide a valid URL";
pub const VALID_URL_SHCEME: &'static str = "Please provide url only with scheme HTTPS";
