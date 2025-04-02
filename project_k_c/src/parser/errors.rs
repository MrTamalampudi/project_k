use crate::{keywords::TokenType, lexer::Token};

use super::Parser;

pub fn collect_prerequisite_path_error(parser: &mut Parser) {
    parser.ctx.errors.insert_parsing_error(
        format!("No such testcase exists in directory"),
        &parser.lexer.next_token(),
    );
}

pub fn collect_capability_key_error(token: &Token, parser: &mut Parser) {
    parser
        .ctx
        .errors
        .insert_parsing_error(format!("Expected a valid capability key"), &token);

    //consume till new line token beacause of error
    //consume assign token
    match parser.lexer.peek_token() {
        TokenType::ASSIGN_OP => {
            parser.lexer.next_token();
        } // consume assign token
        x @ _ => panic!("Expected Assign token got {}", x),
    }
    //consume capability value token
    match parser.lexer.peek_token() {
        TokenType::STRING(string) => {
            parser.lexer.next_token();
        }
        TokenType::IDENTIFIER(ident) => {
            parser.lexer.next_token();
        }
        x @ _ => panic!("Expected capability value"),
    }
}
