use testcase::parse_testcase;
use testsuite::parse_testsuite;

use crate::ast::EntryPoint;
use crate::lexer::Lexer;
use crate::{CompilationContext, TokenType};

pub mod testcase;
pub mod testplan;
pub mod testsuite;

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
        let token_type = self.lexer.peek_token();
        let entrypoint = match token_type {
            TokenType::TESTCASE => EntryPoint::TESTCASE(parse_testcase(self)),
            TokenType::TESTSUITE => EntryPoint::TESTSUITE(parse_testsuite(self)),
            TokenType::TESTPLAN => todo!(),
            _ => return,
        };
        self.ctx.program.set_entrypoint(entrypoint);
    }

    pub fn set_lexer(&mut self, lexer: Lexer) {
        *self.lexer = lexer;
    }
}
