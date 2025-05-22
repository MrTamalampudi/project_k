use crate::ast::Location;
use crate::keywords::TokenType;
use crate::token::Token;
use crate::CompilationContext;
use std::iter::Peekable;
use std::str::Chars;
use std::{fmt, path::PathBuf};
use unicode_ident::{is_xid_continue, is_xid_start};

const WHITESPACE: char = ' ';
const NEW_LINE: char = '\n';
const DOUBLE_QUOTE: char = '\"';
const HASH_TAG: char = '#';
const BACKSLASH: char = '\\';
const FORWARDSLASH: char = '/';
const DOLLAR: char = '$';
const ASSIGN: char = '=';
const UNDERLINE: char = '_';

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug)]
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
    source_code: String,
    source_path: PathBuf,
    ctx: &'a mut CompilationContext,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source_code: String, ctx: &'a mut CompilationContext) -> Tokenizer<'a> {
        Tokenizer {
            source_code,
            source_path: ctx.path.clone(),
            ctx,
        }
    }

    fn get_source_path_as_string(&self) -> String {
        self.source_path.to_str().unwrap().to_string()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let source_code = self.source_code.clone();
        let mut state = State {
            peekable: source_code.chars().peekable(),
            location: Location::new(1, 1),
        };

        let mut tokens: Vec<Token> = vec![];
        self.get_token(&mut state, &mut tokens);
        tokens
    }

    fn error(
        &mut self,
        message: String,
        start_location: Location,
        end_location: Location,
        source_path: String,
    ) {
        self.ctx
            .errors
            .insert_error(message, start_location, end_location, source_path);
    }

    //tokenization starts from here
    pub fn get_token(&mut self, state: &mut State, tokens: &mut Vec<Token>) {
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE => Tokenizer::counsume_unwanted_token(state),
                &NEW_LINE => self.consume_operator_token(TokenType::NEW_LINE, state, tokens, 1),
                &DOUBLE_QUOTE => self.consume_string_token(state, tokens),
                &ASSIGN => self.consume_operator_token(TokenType::ASSIGN_OP, state, tokens, 1),
                &FORWARDSLASH => self.consume_comments(state),
                &HASH_TAG => {
                    state.next(); // consume '#' token
                    self.consume_identifier(state, tokens);
                }
                ch if is_xid_start(*ch) || ch == &UNDERLINE => {
                    self.consume_identifier(state, tokens)
                }
                _ => {
                    self.error(
                        "Unexpected character".to_string(),
                        state.location,
                        {
                            state.next(); //consume unexpected char
                            state.location
                        },
                        self.get_source_path_as_string(),
                    );
                }
            };
        }
        //after file reaching eod we are explicitly adding eof token here
        tokens.push(Token::new(
            TokenType::EOF,
            Location::dummy(),
            Location::dummy(),
            self.get_source_path_as_string(),
        ));
    }

    fn consume_comments(&mut self, state: &mut State) {
        let start = state.location;
        state.next(); //consume first '/' of a comment
        if let Some(ch) = state.peek() {
            match ch {
                &FORWARDSLASH => {
                    state.next();
                } //consume second '/' of a comment
                _ => self.error(
                    "Expected '/'".to_string(),
                    start,
                    {
                        state.next();
                        state.location
                    },
                    self.get_source_path_as_string(),
                ),
            }
        }

        //consume till it notices a line end '\n'
        while let Some(ch) = state.next() {
            if ch == NEW_LINE {
                break;
            }
        }
    }

    fn consume_identifier(&self, state: &mut State, tokens: &mut Vec<Token>) {
        let start = state.location;
        let mut string: String = String::new();
        let mut token_type: TokenType = TokenType::NONE;

        if let Some(ch) = state.peek() {
            if is_xid_start(*ch) || *ch == '_' {
                string.push(*ch);
                state.next(); //consume first char of ident
            }
        }

        while let Some(ch) = state.peek() {
            match token_type {
                TokenType::NONE => {
                    if is_xid_continue(*ch) {
                        string.push(*ch);
                        state.next();
                    } else {
                        token_type = TokenType::from_string(string.as_str())
                    }
                }
                _ => break,
            }
        }

        tokens.push(Token::new(
            token_type,
            start,
            state.location,
            self.get_source_path_as_string(),
        ))
    }

    fn consume_operator_token(
        &self,
        token_type: TokenType,
        state: &mut State,
        tokens: &mut Vec<Token>,
        token_length: u8,
    ) {
        let start = state.location;
        //consume until token length
        for i in 1..=token_length {
            state.next();
        }
        tokens.push(Token::new(
            token_type,
            start,
            state.location,
            self.get_source_path_as_string(),
        ));
    }

    fn consume_eof(&self, state: &mut State, tokens: &mut Vec<Token>) {
        tokens.push(Token::new(
            TokenType::EOF,
            state.location,
            state.location,
            self.get_source_path_as_string(),
        ));
    }

    fn counsume_unwanted_token(state: &mut State) {
        state.next();
    }

    fn consume_string_token(&self, state: &mut State, tokens: &mut Vec<Token>) {
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
            self.get_source_path_as_string(),
        ));
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub tokens: Vec<Token>,
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

    pub fn double_peek_token(&self) -> &TokenType {
        self.tokens
            .get(self.cursor_position + 1)
            .map_or(&TokenType::EOF, |x| &x.token_type)
    }

    pub fn next_token(&mut self) -> Token {
        if self.cursor_position >= self.tokens.len() {
            Token::new(
                TokenType::EOF,
                Location::new(0, 0),
                Location::new(0, 0),
                String::from(""),
            )
        } else {
            self.cursor_position += 1;
            self.tokens[self.cursor_position - 1].clone()
        }
    }

    pub fn shift_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens.splice(0..0, tokens);
    }
}
