use errors::ParserError;
use testcase::parser_slr;

use crate::error_handling::ErrorInfo;
use crate::lexer::Lexer;
use crate::{CompilationContext, TokenType};

pub mod errors;
pub mod locator;
pub mod testcase;
pub mod testplan;

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

    pub fn error(&mut self, message: ParserError) {
        let token = self.lexer.next_token();
        self.ctx.errors.errors.push(ErrorInfo {
            message: format!("{message}"),
            start_location: token.get_start_location(),
            end_location: token.get_end_location(),
            source_path: token.get_source_path(),
        });
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
