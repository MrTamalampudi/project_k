use crate::{keywords::NTokenType, CompilationContext};
use logos::Lexer;
use testcase::parser_slr;

mod actions;
mod errors;
mod errorss;
pub mod testcase;
pub mod translator_stack;

#[derive(Debug)]
pub struct Parser<'a, 'b> {
    lexer: Lexer<'b, NTokenType>,
    ctx: &'a mut CompilationContext,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(lexer: Lexer<'b, NTokenType>, ctx: &'a mut CompilationContext) -> Parser<'a, 'b> {
        Parser { lexer, ctx }
    }
    pub fn parse(&mut self) {
        parser_slr(self);
        //println!("errors {:#?}", self.ctx.errors.errors);
        // let token_type = self.lexer.peek_token();
        // let entrypoint = match token_type {
        //     TokenType::TESTCASE => EntryPoint::TESTCASE(parse_testcase(self)),
        //     TokenType::TESTSUITE => EntryPoint::TESTSUITE(parse_testsuite(self)),
        //     TokenType::TESTPLAN => todo!(),
        //     _ => return,
        // };
        // self.ctx.program.set_entrypoint(entrypoint);
    }
}
