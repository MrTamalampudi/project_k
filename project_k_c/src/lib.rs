use enums::Capabilities;
use error_handling::ErrorManager;
use keywords::TokenType;
use lexer::{Lexer, Tokenizer};
use parser::testcase;
use parser::testcase::parser_slr;
use parser::Parser;
use std::env;
use std::fmt;
use std::fs;
use std::path::Display;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use token::Token;
use utils::get_parent;

use crate::program::Program;

pub mod ast;
pub mod class;
pub mod class_macro;
pub mod engine;
pub mod enums;
pub mod error_handling;
pub mod keywords;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod program;
pub mod token;
pub mod utils;

#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub path: PathBuf,
    pub errors: ErrorManager,
    pub program: Program,
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
            program: Program::new(),
        }
    }

    pub fn set_path(&mut self, path: &PathBuf) {
        self.path = path.clone();
    }

    pub fn get_parent_path(&self) -> PathBuf {
        self.path.parent().unwrap().to_path_buf()
    }
}

fn read_file_to_string(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(string) => string,
        Err(error) => panic!("{:#?}", error),
    }
}

fn source_code_to_tokens(source_code: String, ctx: &mut CompilationContext) -> Vec<Token> {
    Tokenizer::new(source_code, ctx).tokenize()
}

pub fn source_code_to_lexer(source_code: String, ctx: &mut CompilationContext) -> Lexer {
    let tokens = source_code_to_tokens(source_code, ctx);
    //println!("tokens {:#?}", tokens);
    Lexer::from_tokens(tokens)
}

pub fn compile_for_errors(ctx: &mut CompilationContext) {
    let source_code = read_file_to_string(&ctx.path);
    let mut lexer = source_code_to_lexer(source_code, ctx);
    //parser_slr(lexer.tokens.clone());
    //println!("{:#?}", lexer.tokens);
    Parser::new(&mut lexer, ctx).parse();
}

pub fn compile(ctx: &mut CompilationContext) {
    compile_for_errors(ctx);
    //println!("{:#?}", ctx.program);
    //println!("{:#?}", ctx.errors);
    //execute(ctx.program.clone());
}
