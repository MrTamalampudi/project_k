use crate::ast::Location;
use crate::keywords::TokenType;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use unicode_ident::{is_xid_continue, is_xid_start};

const WHITESPACE: char = ' ';
const NEW_LINE: char = '\n';
const DOUBLE_QUOTE: char = '\"';
const HASH_TAG: char = '#';
const BACKSLASH: char = '\\';
const DOLLAR: char = '$';
const ASSIGN: char = '=';

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Token {
    token_type: TokenType,
    start: Location,
    end: Location,
}

impl Token {
    fn new(token_type: TokenType, start: Location, end: Location) -> Self {
        Self {
            token_type,
            start,
            end,
        }
    }

    pub fn get_start_location(&self) -> Location {
        self.start
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

#[derive(Debug)]
pub struct TokenizerError {
    pub message: String,
    pub location: Location,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.message, self.location,)
    }
}

impl std::error::Error for TokenizerError {}

pub struct State<'a> {
    peekable: Peekable<Chars<'a>>,
    pub location: Location,
}

impl State<'_> {
    pub fn next(&mut self) -> Option<char> {
        match self.peekable.next() {
            None => None,
            Some(s) => {
                if s == NEW_LINE {
                    self.location.line += 1;
                    self.location.column = 1;
                } else {
                    self.location.column += 1;
                }
                Some(s)
            }
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.peekable.peek()
    }
}

pub struct Tokenizer<'a> {
    source_code: &'a String,
}

type TokenizerResult = Result<(), TokenizerError>;

impl<'a> Tokenizer<'a> {
    pub fn new(source_code: &'a String) -> Self {
        Self { source_code }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ()> {
        let mut state = State {
            peekable: self.source_code.chars().peekable(),
            location: Location::new(1, 1),
        };

        let mut tokens: Vec<Token> = vec![];
        Tokenizer::get_token(&mut state, &mut tokens);
        Ok(tokens)
    }

    pub fn get_token(state: &mut State, tokens: &mut Vec<Token>) -> TokenizerResult {
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE => Tokenizer::counsume_unwanted_token(state)?,
                &NEW_LINE => Tokenizer::counsume_unwanted_token(state)?,
                &DOUBLE_QUOTE => Tokenizer::consume_string_token(state, tokens)?,
                &HASH_TAG => Tokenizer::consume_highlevel_tokens(state, tokens)?,
                &DOLLAR => Tokenizer::consume_identifier(state, tokens)?,
                &ASSIGN => Tokenizer::consume_operator_token(TokenType::ASSIGN_OP, state, tokens),
                'A'..='Z' | 'a'..='z' => Tokenizer::consume_keyword(state, tokens)?,
                _ => Tokenizer::consume_eof(state, tokens),
            };
        }
        Ok(())
    }

    fn consume_operator_token(token_type: TokenType, state: &mut State, tokens: &mut Vec<Token>) {
        state.next(); //consume token
        tokens.push(Token::new(
            token_type,
            state.location,
            state.location.next_column(),
        ));
    }

    fn consume_identifier(state: &mut State, tokens: &mut Vec<Token>) -> TokenizerResult {
        let start = state.location;
        state.next(); //consume $
        let mut string: String = String::new();
        match state.peek() {
            Some(ch) => {
                if is_xid_start(*ch) {
                    string.push(*ch);
                    state.next(); //consume char
                }
            }
            None => {}
        }

        loop {
            let ch = state.peek();
            match ch {
                Some(ch) => {
                    if is_xid_continue(*ch) && string.len() > 0 {
                        string.push(*ch);
                        state.next(); // consume char
                    } else {
                        break;
                    }
                }
                None => {}
            }
        }

        if string.len() > 0 {
            tokens.push(Token::new(
                TokenType::IDENTIFIER(string),
                start,
                state.location,
            ));
        }

        Ok(())
    }

    fn consume_eof(state: &mut State, tokens: &mut Vec<Token>) {
        tokens.push(Token::new(TokenType::EOF, state.location, state.location));
    }

    fn consume_highlevel_tokens(
        state: &mut State,
        tokens: &mut Vec<Token>,
    ) -> Result<(), TokenizerError> {
        let start_location = state.location;
        state.next(); // consume '#' token
        let mut string: String = String::new();
        let mut token_type = TokenType::NONE;
        while let Some(ch) = state.peek() {
            match token_type {
                TokenType::NONE => {
                    if !('a'..='z').contains(&ch) {
                        return Err(TokenizerError {
                            message: String::from("unexpected tokenfcdff"),
                            location: state.location,
                        });
                    }
                    string.push(*ch);
                    state.next();
                    token_type = TokenType::from_string(string.to_lowercase().as_str());
                }
                _ => match ch {
                    'a'..='z' | 'A'..='Z' => {
                        return Err(TokenizerError {
                            message: format!("Expected nested space {string} before EOF."),
                            location: state.location,
                        });
                    }
                    _ => break,
                },
            }
        }
        tokens.push(Token {
            token_type,
            end: state.location,
            start: start_location,
        });
        Ok(())
    }

    fn counsume_unwanted_token(state: &mut State) -> Result<(), TokenizerError> {
        state.next();
        Ok(())
    }

    fn consume_string_token(
        state: &mut State,
        tokens: &mut Vec<Token>,
    ) -> Result<(), TokenizerError> {
        let start_location = state.location;
        state.next(); //consume starting quote
        let mut string: String = String::new();
        while let Some(s) = state.next() {
            if s == BACKSLASH {
                //consume backslash
                match state.next() {
                    Some(ch) => string.push(ch),
                    None => {}
                }
            } else if s != DOUBLE_QUOTE {
                string.push(s);
            } else {
                break;
            }
        }
        tokens.push(Token::new(
            TokenType::STRING(string),
            start_location,
            state.location,
        ));

        Ok(())
    }

    fn consume_keyword(state: &mut State, tokens: &mut Vec<Token>) -> Result<(), TokenizerError> {
        let start: Location = state.location;
        let mut string: String = String::new();
        let mut token_type: TokenType = TokenType::NONE;
        while let Some(s) = state.next() {
            match token_type {
                TokenType::NONE => {
                    if s == DOUBLE_QUOTE || s == NEW_LINE {
                        return Err(TokenizerError {
                            message: String::from("unexpected"),
                            location: state.location,
                        });
                    }
                    string.push(s);
                    token_type = TokenType::from_string(string.to_lowercase().as_str());
                }
                _ => break,
            }
        }
        tokens.push(Token::new(token_type, start, state.location));
        Ok(())
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    cursor_position: usize,
}

impl Lexer {
    pub fn from_tokens(tokens: Vec<Token>) -> Lexer {
        Lexer {
            tokens,
            cursor_position: 0,
        }
    }

    pub fn peek_token(&self) -> &TokenType {
        self.tokens
            .get(self.cursor_position)
            .map_or(&TokenType::EOF, |x| &x.token_type)
    }

    pub fn next_token(&mut self) -> Token {
        if self.cursor_position >= self.tokens.len() {
            Token::new(TokenType::EOF, Location::new(0, 0), Location::new(0, 0))
        } else {
            self.cursor_position += 1;
            self.tokens[self.cursor_position - 1].clone()
        }
    }
}
