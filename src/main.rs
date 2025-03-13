use ast::Testcase;
use engine::execute_test_case;
use keywords::TokenType;
use lexer::{Lexer, Tokenizer};
use parser::parse_test_case;
use std::env;
use std::fs;
use std::process::Command;
use std::process::Output;
use std::str::FromStr;

mod actions;
mod ast;
mod engine;
pub mod enums;
pub mod keywords;
pub mod lexer;
mod parser;

enum ExecutionType {
    TESTCASE,
    TESTSUITE,
    TESTPLAN,
}

struct CompilationContext {
    execution_type: ExecutionType,
    project_root: String,
}

impl CompilationContext {
    fn new(project_root: String, execution_type: ExecutionType) -> Self {
        Self {
            execution_type,
            project_root,
        }
    }
}

fn read_file_to_string(path: &String) -> String {
    match fs::read_to_string(path) {
        Ok(string) => string,
        Err(error) => panic!("{:#?}", error),
    }
}

fn source_code_to_lexer(source_code: String) -> Lexer {
    let tokens = match Tokenizer::new(&source_code).tokenize() {
        Ok(tokenss) => tokenss,
        Err(tokenss) => panic!("error, {:#?}", tokenss),
    };
    println!("{:#?}", tokens);
    Lexer::from_tokens(tokens)
}

fn compile(entry_point: &String) {
    let source_code = read_file_to_string(entry_point);
    let mut lexer = source_code_to_lexer(source_code);

    let file_type_token = lexer.peek_token();

    let testcase: Testcase = match file_type_token {
        TokenType::TESTCASE => {
            let project_root = Command::new("dirname")
                .args([entry_point])
                .output()
                .expect("Error")
                .stdout;
            let project_root = String::from_utf8(project_root)
                .expect("error")
                .trim()
                .to_string();
            let compilation_context =
                CompilationContext::new(project_root, ExecutionType::TESTCASE);
            parse_test_case(&mut lexer, &compilation_context)
        }
        _ => panic!("testcase"),
    };

    println!("{:#?}", testcase);

    //execute_test_case(testcase);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => panic!("please provide a file path"),
    };
    compile(source_path);
    //println!("{:#?}", status);
    //compile(String::from("./new.ll"));
}
