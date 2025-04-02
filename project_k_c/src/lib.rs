use ast::Program;
use ast::TestCase;
use engine::execute;
use engine::execute_test_case;
use error_handling::ErrorManager;
use keywords::TokenType;
use lexer::Token;
use lexer::{Lexer, Tokenizer};
use parser::testcase;
use parser::Parser;
use std::env;
use std::fmt;
use std::fs;
use std::path::Display;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use utils::get_parent;

mod actions;
pub mod ast;
mod engine;
pub mod enums;
pub mod error_handling;
pub mod keywords;
pub mod lexer;
mod parser;
pub mod utils;

enum ExecutionType {
    TESTCASE,
    TESTSUITE,
    TESTPLAN,
}

#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub path: PathBuf,
    pub errors: ErrorManager,
    pub program: Program,
    pub lsp: bool,
}

impl fmt::Display for CompilationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl CompilationContext {
    pub fn new(entry_path: PathBuf, lsp: bool) -> CompilationContext {
        CompilationContext {
            path: entry_path,
            errors: ErrorManager::new(),
            program: Program::new(),
            lsp,
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

fn source_code_to_lexer(source_code: String, ctx: &mut CompilationContext) -> Lexer {
    let tokens = source_code_to_tokens(source_code, ctx);
    println!("tokens {:#?}", tokens);
    Lexer::from_tokens(tokens)
}

pub fn compile_for_errors(ctx: &mut CompilationContext) {
    let source_code = read_file_to_string(&ctx.path);
    let mut lexer = source_code_to_lexer(source_code, ctx);
    Parser::new(&mut lexer, ctx).parse();
}

pub fn compile(ctx: &mut CompilationContext) {
    compile_for_errors(ctx);
    println!("{:#?}", ctx.program);
    //execute(ctx.program.clone());
}
