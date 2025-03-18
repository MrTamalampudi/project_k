use crate::enums::{Browser, LexingMode};
use crate::error_handling::{ErrorInfo, ErrorManager};
use crate::keywords::TokenType;
use crate::CompilationContext;
use crate::{ast::Location, enums::Capabilities};
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

macro_rules! consume_keyword_token {
    (
        $token_enum:ident,
        $state:ident,
        $tokens:ident,
        $source_path:ident
    ) => {
        let start: Location = $state.location;
        let mut string: String = String::new();
        let mut provided_type: $token_enum = $token_enum::NONE;
        while let Some(s) = $state.peek() {
            match provided_type {
                $token_enum::NONE => {
                    if s == &DOUBLE_QUOTE || s == &NEW_LINE {
                        panic!("Unexpected")
                    }
                    string.push(*s);
                    $state.next();
                    provided_type = $token_enum::from_string(string.to_lowercase().as_str());
                }
                _ => break,
            }
        }

        let token_type = provided_type.match_token_type();
        $tokens.push(Token::new(token_type, start, $state.location, $source_path));
    };
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Token {
    token_type: TokenType,
    start: Location,
    end: Location,
    source_path: String,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Token {
    fn new(token_type: TokenType, start: Location, end: Location, source_path: String) -> Self {
        Self {
            token_type,
            start,
            end,
            source_path,
        }
    }

    pub fn get_start_location(&self) -> Location {
        self.start
    }

    pub fn get_end_location(&self) -> Location {
        self.end
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
    source_path: String,
    ctx: &'a mut CompilationContext,
}

type TokenizerResult = Result<(), TokenizerError>;

impl<'a> Tokenizer<'a> {
    pub fn new(source_code: String, source_path: String, ctx: &'a mut CompilationContext) -> Self {
        Self {
            source_code,
            source_path,
            ctx,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ()> {
        let source_code = self.source_code.clone();
        let mut state = State {
            peekable: source_code.chars().peekable(),
            location: Location::new(1, 1),
        };

        let mut tokens: Vec<Token> = vec![];
        self.get_token(&mut state, &mut tokens);
        Ok(tokens)
    }

    //tokenization starts from here
    pub fn get_token(&mut self, state: &mut State, tokens: &mut Vec<Token>) {
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE | &NEW_LINE => Tokenizer::counsume_unwanted_token(state),
                &HASH_TAG => self.consume_highlevel_tokens(state, tokens),
                _ => self.consume_eof(state, tokens),
            };
        }
    }

    fn consume_operator_token(
        &self,
        token_type: TokenType,
        state: &mut State,
        tokens: &mut Vec<Token>,
    ) {
        tokens.push(Token::new(
            token_type,
            state.location,
            {
                state.next();
                state.location
            },
            self.source_path.clone(),
        ));
    }

    //using this fn for consuming both prerequisite testcase and variable identifiers
    fn consume_identifier(&self, state: &mut State, tokens: &mut Vec<Token>) {
        let start = state.location;
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
                self.source_path.clone(),
            ));
        }
    }

    fn consume_eof(&self, state: &mut State, tokens: &mut Vec<Token>) {
        tokens.push(Token::new(
            TokenType::EOF,
            state.location,
            state.location,
            self.source_path.clone(),
        ));
    }

    fn consume_highlevel_tokens(&mut self, state: &mut State, tokens: &mut Vec<Token>) {
        let start_location = state.location;
        state.next(); // consume '#' token
        let mut string: String = String::new();
        let mut lexing_mode = LexingMode::NONE;
        while let Some(ch) = state.peek() {
            match lexing_mode {
                LexingMode::NONE => {
                    if ch == &DOUBLE_QUOTE || ch == &NEW_LINE {
                        panic!("Unexpected token");
                    }
                    string.push(*ch);
                    state.next();
                    lexing_mode = LexingMode::from_string(string.to_lowercase().as_str());
                }
                _ => break,
            }
        }
        let token_type = lexing_mode.match_token_type();
        tokens.push(Token {
            token_type,
            end: state.location,
            start: start_location,
            source_path: self.source_path.clone(),
        });

        match lexing_mode {
            LexingMode::PREREQUISITE => self.consume_prerequisite_tokens(state, tokens),
            LexingMode::TESTSTEPS => self.consume_teststep_tokens(state, tokens),
            LexingMode::CAPABILITIES => self.consume_capabilities_tokens(state, tokens),
            _ => {}
        }
    }

    fn consume_capabilities_tokens(&self, state: &mut State, tokens: &mut Vec<Token>) {
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE | &NEW_LINE => Tokenizer::counsume_unwanted_token(state),
                &HASH_TAG => break,
                'A'..='Z' | 'a'..='z' => self.consume_capability(state, tokens),
                _ => self.consume_eof(state, tokens),
            };
        }
    }

    fn consume_capability(&self, state: &mut State, tokens: &mut Vec<Token>) {
        //didnt use consume_keyword_token macro beacause we need capability_type further
        let start: Location = state.location;
        let mut capability_string: String = String::new();
        let mut capability_type: Capabilities = Capabilities::NONE;
        while let Some(s) = state.peek() {
            match capability_type {
                Capabilities::NONE => {
                    if s == &DOUBLE_QUOTE || s == &NEW_LINE {
                        panic!("Unexpected")
                    }
                    capability_string.push(*s);
                    state.next();
                    capability_type =
                        Capabilities::from_string(capability_string.to_lowercase().as_str());
                }
                _ => break,
            }
        }
        let token_type = capability_type.match_token_type();
        tokens.push(Token::new(
            token_type,
            start,
            state.location,
            self.source_path.clone(),
        ));

        //consume till capability value
        //browser = chrome
        //          ^ consume upto here
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE | &NEW_LINE => Tokenizer::counsume_unwanted_token(state),
                &ASSIGN => self.consume_operator_token(TokenType::ASSIGN_OP, state, tokens),
                'A'..='Z' | 'a'..='z' | &DOUBLE_QUOTE => break,
                _ => self.consume_eof(state, tokens),
            };
        }

        match capability_type {
            Capabilities::BROWSER => self.consume_browser_capability_token(state, tokens),
            Capabilities::DRIVERURL => self.consume_driverurl_capability_token(state, tokens),
            _ => todo!(),
        }
    }

    fn consume_driverurl_capability_token(&self, state: &mut State, tokens: &mut Vec<Token>) {
        self.consume_string_token(state, tokens);
    }

    fn consume_browser_capability_token(&self, state: &mut State, tokens: &mut Vec<Token>) {
        let source_path = self.source_path.clone();
        consume_keyword_token!(Browser, state, tokens, source_path);
    }

    fn consume_teststep_tokens(&self, state: &mut State, tokens: &mut Vec<Token>) {
        while let Some(cha) = state.peek() {
            match cha {
                &WHITESPACE | &NEW_LINE => Tokenizer::counsume_unwanted_token(state),
                &DOUBLE_QUOTE => self.consume_string_token(state, tokens),
                &HASH_TAG => break,
                &DOLLAR => {
                    state.next(); // consume dollar sign
                    self.consume_identifier(state, tokens)
                }
                &ASSIGN => self.consume_operator_token(TokenType::ASSIGN_OP, state, tokens),
                'A'..='Z' | 'a'..='z' => self.consume_keyword(state, tokens),
                _ => self.consume_eof(state, tokens),
            };
        }
    }

    fn consume_prerequisite_tokens(&self, state: &mut State, tokens: &mut Vec<Token>) {
        while let Some(ch) = state.peek() {
            if is_xid_start(*ch) {
                self.consume_identifier(state, tokens)
            } else if ch == &WHITESPACE || ch == &NEW_LINE {
                Tokenizer::counsume_unwanted_token(state)
            } else {
                break;
            };
        }
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
            self.source_path.clone(),
        ));
    }

    fn consume_keyword(&self, state: &mut State, tokens: &mut Vec<Token>) {
        let source_path = self.source_path.clone();
        consume_keyword_token!(TokenType, state, tokens, source_path);
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
}
