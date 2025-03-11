use engine::execute_test_case;
use keywords::TokenType;
use lexer::{Lexer, Tokenizer};
use parser::parse_test_case;
use std::env;
use std::error::Error;
use std::fs;

mod actions;
mod ast;
mod engine;
pub mod keywords;
pub mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut argss: Vec<String> = env::args().collect();
    println!("{:#?}", argss);
    //env::set_var("RUST_BACKTRACE", "1");
    let source_code_string = fs::read_to_string("./new.ll")?;
    let tokens = match Tokenizer::new(&source_code_string).tokenize() {
        Ok(tokenss) => tokenss,
        Err(tokenss) => panic!("error, {:#?}", tokenss),
    };
    println!("{:#?}", tokens);

    // let mut lexer = Lexer::from_tokens(tokens);

    // //check file type
    // let file_type_token = lexer.peek_token();

    // match file_type_token {
    //     TokenType::TESTCASE => parse_test_case(&mut lexer),
    //     _ => eprintln!("some error"),
    // }
    // execute_test_case(test_case);
    Ok(())
}
