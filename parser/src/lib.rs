use error_handler::ErrorManager;
use keywords::TokenType;
use lexer::{Lexer, Tokenizer};
use parser::Parser;
use std::fmt;
use std::path::PathBuf;
use token::Token;

use crate::program::Program;

pub mod error_handler;
pub mod keywords;
pub mod lexer;
pub mod parser;
pub mod program;
pub mod token;
//pub use crate::parser::grammar::lalr_generate;

#[derive(Debug, Clone)]
pub struct CompilationContext {
    ///file path
    pub path: PathBuf,
    pub errors: ErrorManager,
    pub ast: Program,
}

impl fmt::Display for CompilationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl CompilationContext {
    pub fn new(entry_path: PathBuf) -> CompilationContext {
        CompilationContext {
            path: entry_path,
            errors: ErrorManager::new(),
            ast: Program::new(),
        }
    }

    pub fn set_path(&mut self, path: &PathBuf) {
        self.path = path.clone();
    }

    pub fn get_parent_path(&self) -> PathBuf {
        self.path.parent().unwrap().to_path_buf()
    }
}

fn source_code_to_tokens(source_code: String, ctx: &mut CompilationContext) -> Vec<Token> {
    Tokenizer::new(source_code, ctx).tokenize()
}

pub fn source_code_to_lexer(source_code: String, ctx: &mut CompilationContext) -> Lexer {
    let tokens = source_code_to_tokens(source_code, ctx);
    Lexer::from_tokens(tokens)
}

pub fn parse(source: String, ctx: &mut CompilationContext) {
    let mut lexer = source_code_to_lexer(source, ctx);
    Parser::new(&mut lexer, ctx).parse();
}
