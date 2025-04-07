use crate::{keywords::TokenType, lexer::Token};
use core::fmt::{self, Formatter, Write};

use super::{testcase::consume_till_new_line_or_eof_token, Parser};

pub fn collect_prerequisite_path_error(parser: &mut Parser) {
    parser.ctx.errors.insert_parsing_error(
        format!("No such testcase exists in directory"),
        &parser.lexer.next_token(),
    );
}

pub fn collect_capability_key_error(parser: &mut Parser) {
    parser.error(ParserError::CAPABILITY_KEY);
    consume_till_new_line_or_eof_token(parser);
}

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
