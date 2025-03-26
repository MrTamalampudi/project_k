use testcase::parse_test_case;

use crate::ast::EntryPoint;
use crate::lexer::Lexer;
use crate::{CompilationContext, TokenType};

pub mod testcase;
pub mod testplan;
pub mod testsuite;

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
            TokenType::TESTCASE => EntryPoint::TESTCASE(parse_test_case(self)),
            TokenType::TESTSUITE => todo!(),
            TokenType::TESTPLAN => todo!(),
            _ => return,
        };
        self.ctx.program.set_entrypoint(entrypoint);
    }
}
