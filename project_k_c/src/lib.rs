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
use std::process::Command;

mod actions;
pub mod ast;
mod engine;
pub mod enums;
pub mod error_handling;
pub mod keywords;
pub mod lexer;
mod parser;

enum ExecutionType {
    TESTCASE,
    TESTSUITE,
    TESTPLAN,
}

#[derive(Debug, Clone)]
pub struct CompilationContext {
    parent_path: String,
    path: String,
    errors: ErrorManager,
    pub program: Program,
}

impl fmt::Display for CompilationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl CompilationContext {
    pub fn new(entry_path: String) -> CompilationContext {
        CompilationContext {
            parent_path: entry_path.clone(),
            path: entry_path,
            errors: ErrorManager::new(),
            program: Program::new(),
        }
    }

    pub fn set_project_root(&mut self, path: String) {
        self.path = path;
    }
}

fn read_file_to_string(path: &String) -> String {
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
    Lexer::from_tokens(tokens)
}

pub fn compile(entry_point: &String, ctx: &mut CompilationContext) {
    let source_code = read_file_to_string(entry_point);
    let mut lexer = source_code_to_lexer(source_code, ctx);
    Parser::new(&mut lexer, ctx).parse();
    execute(ctx.program.clone());
}
