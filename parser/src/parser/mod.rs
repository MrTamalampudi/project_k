use testcase::parser_slr;

use crate::lexer::Lexer;
use crate::{CompilationContext, TokenType};

mod actions;
mod errors;
mod errorss;
pub mod testcase;
pub mod translator_stack;

#[derive(Debug)]
pub struct Parser<'a, 'b> {
    lexer: &'a mut Lexer,
    ctx: &'b mut CompilationContext,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(lexer: &'a mut Lexer, ctx: &'b mut CompilationContext) -> Parser<'a, 'b> {
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

    pub fn set_lexer(&mut self, lexer: Lexer) {
        *self.lexer = lexer;
    }
}

pub fn consume_new_line_token(parser: &mut Parser) {
    match parser.lexer.peek_token() {
        TokenType::NEW_LINE => {
            parser.lexer.next_token();
        }
        _ => (),
    }
}
