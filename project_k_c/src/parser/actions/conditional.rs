use manodae::error::ParseError;

use crate::{
    ast::testcase::TestCase, class::ConditionalStmtAction,
    parser::translator_stack::TranslatorStack, token::Token,
};

pub struct Conditional;

impl ConditionalStmtAction for Conditional {
    fn IF(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
    fn ELSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
    }
}
