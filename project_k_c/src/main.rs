use ast::Testcase;
use engine::execute_test_case;
use error_handling::ErrorManager;
use keywords::TokenType;
use lexer::{Lexer, Tokenizer};
use parser::parse_test_case;
use std::env;
use std::fs;
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

#[derive(Debug)]
pub struct CompilationContext {
    path: String,
    errors: ErrorManager,
}

impl CompilationContext {
    pub fn new(path: String) -> Self {
        Self {
            path,
            errors: ErrorManager::new(),
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

fn source_code_to_lexer(
    source_code: String,
    source_path: &String,
    ctx: &mut CompilationContext,
) -> Lexer {
    let tokens = match Tokenizer::new(source_code, source_path.clone(), ctx).tokenize() {
        Ok(tokenss) => tokenss,
        Err(tokenss) => panic!("error, {:#?}", tokenss),
    };
    println!("{:#?}", tokens);
    Lexer::from_tokens(tokens)
}

fn compile(entry_point: &String, ctx: &mut CompilationContext) {
    let source_code = read_file_to_string(entry_point);
    let mut lexer = source_code_to_lexer(source_code, entry_point, ctx);

    let file_type_token = lexer.peek_token();

    // let testcase: Testcase = match file_type_token {
    //     TokenType::TESTCASE => {
    //         let project_root = Command::new("dirname")
    //             .args([entry_point])
    //             .output()
    //             .expect("Error")
    //             .stdout;
    //         let project_root = String::from_utf8(project_root)
    //             .expect("error")
    //             .trim()
    //             .to_string();
    //         ctx.set_project_root(project_root);
    //         parse_test_case(&mut lexer, ctx)
    //     }
    //     _ => panic!("testcase"),
    // };

    println!("{:#?}", ctx.errors);

    // println!("{:#?}", testcase);

    //execute_test_case(testcase);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => panic!("please provide a file path"),
    };
    let mut ctx = CompilationContext::new(source_path.clone());
    compile(source_path, &mut ctx);
    //println!("{:#?}", status);
    //compile(String::from("./new.ll"));
}
