use std::cmp::Ordering;

use project_k::{enums::Capabilities, keywords::TokenType, lexer::Token};
use tower_lsp::lsp_types::{CompletionItem, MessageType, Position};

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
    back: &Backend,
    tokens: &Vec<Token>,
    position: &Position,
) -> Vec<CompletionItem> {
    let index: usize = match tokens.binary_search_by(|token| token_comparator(token, &position)) {
        Ok(index) => index,
        Err(index) => index,
    };
    let filetype = tokens.get(0).unwrap();
    let highlevel_token = get_highlevel_token(filetype.get_token_type(), tokens, index);
    back.client
        .show_message(
            MessageType::INFO,
            format!("tokendds {:#?}", highlevel_token),
        )
        .await;
    //highlevel_token
    complete(highlevel_token, tokens, index)
}

fn complete(highlevel_token: TokenType, tokens: &Vec<Token>, index: usize) -> Vec<CompletionItem> {
    match highlevel_token {
        TokenType::CAPABILITIES => complete_capabilities(tokens, index),
        TokenType::TESTSTEPS => complete_teststeps(tokens, index),
        _ => todo!(),
    }
}

fn complete_teststeps(tokens: &Vec<Token>, index: usize) -> Vec<CompletionItem> {
    todo!()
}

fn complete_capabilities(tokens: &Vec<Token>, index: usize) -> Vec<CompletionItem> {
    let token_type = tokens.get(index - 1).unwrap().get_token_type();
    if token_type == TokenType::NEW_LINE {
        complete_capbility_key()
    } else if token_type == TokenType::ASSIGN_OP {
        complete_capbility_key()
    } else {
        complete_capbility_key()
    }
}

fn complete_capbility_key() -> Vec<CompletionItem> {
    Capabilities::to_vector()
        .iter()
        .map(|capability| {
            CompletionItem::new_simple(capability.clone(), String::from("Capbilities"))
        })
        .collect()
}

//based on filetype we finding high level token for completion
fn get_highlevel_token(filetype: TokenType, tokens: &Vec<Token>, index: usize) -> TokenType {
    match filetype {
        TokenType::TESTCASE => get_testcase_high_level_token(tokens, index),
        _ => TokenType::NONE,
    }
}

fn get_testcase_high_level_token(tokens: &Vec<Token>, index: usize) -> TokenType {
    let mut index = index;
    let mut token_type = TokenType::NONE;
    while index > 0 && token_type == TokenType::NONE {
        let token = match tokens.get(index) {
            Some(t) => t,
            None => break,
        };
        match token.get_token_type() {
            TokenType::TESTSTEPS | TokenType::CAPABILITIES | TokenType::PREREQUISITE => {
                token_type = token.get_token_type();
            }
            _ => {}
        };

        index -= 1;
    }
    token_type
}
