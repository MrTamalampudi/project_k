use error_handler::ErrorManager;
// use lexer::{Lexer, Tokenizer};
use logos::{Lexer, Logos};
use parser::Parser;
use std::fmt;
use std::path::PathBuf;

use crate::keywords::NTokenType;
use crate::program::Program;

pub mod error_handler;
pub mod keywords;
// pub mod lexer;
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

pub fn source_code_to_lexer<'a>(source_code: &'a str) -> Lexer<'a, NTokenType> {
    NTokenType::lexer(source_code)
}

pub fn parse<'a>(source: &'a str, ctx: &mut CompilationContext) {
    let lexer = source_code_to_lexer(source);
    println!("{:#?}", lexer.clone().collect::<Vec<_>>());
    Parser::new(lexer, ctx).parse();
}
