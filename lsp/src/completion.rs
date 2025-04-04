use std::cmp::Ordering;

use project_k::{keywords::TokenType, lexer::Token};
use tower_lsp::lsp_types::{MessageType, Position};

use crate::Backend;

fn token_comparator(token: &Token, position: &Position) -> Ordering {
    //subtracting 1 from line beacause lsp range starts from 0
    let token_end_line = (token.get_end_location().line - 1) as u32;
    //for this some how it worked
    let token_end_column = (token.get_end_location().column) as u32;
    //check Ordering struct how less & greater have been decided
    if token_end_line == position.line {
        if token_end_column == position.character {
            Ordering::Equal
        } else if token_end_column < position.character {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else if token_end_line < position.line {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub async fn token_by_pos_for_completion(
    backend: &Backend,
    tokens: &Vec<Token>,
    position: &Position,
) -> Token {
    let index: usize = match tokens.binary_search_by(|token| token_comparator(token, &position)) {
        Ok(index) => index,
        Err(index) => index,
    };
    let filetype = tokens.get(0).unwrap();
    let highlevel_token = TokenType::CAPABILITIES;
    tokens.get(index - 1).unwrap().clone()
}
