use std::{cell::RefCell, rc::Rc, thread::park};

use crate::{ast::TestSuite, keywords::TokenType, lexer, utils::get_parent};

use super::{consume_new_line_token, testcase::parse_testcase as parse_tc, Parser};
use crate::{read_file_to_string, source_code_to_lexer};

pub fn parse_testsuite(parser: &mut Parser) -> Rc<TestSuite> {
    let mut testsuite = TestSuite::new();
    //consume #TESTSUITE token
    parser.lexer.next_token();
    consume_new_line_token(parser);
    parse_top_level_items(&mut testsuite, parser);
    parser.ctx.program.push_testsuite(&testsuite)
}

fn parse_top_level_items(testcase: &mut TestSuite, parser: &mut Parser) {
    while let token = parser.lexer.peek_token() {
        match token {
            TokenType::TESTCASE => parse_testcase(testcase, parser),
            TokenType::PREREQUISITE => parse_prerequisite(testcase, parser),
            TokenType::NEW_LINE => {
                parser.lexer.next_token();
            }
            TokenType::EOF => break,
            _ => break,
        }
    }
}

fn parse_testcase(testsuite: &mut TestSuite, parser: &mut Parser) {
    // consume #TESTCASE highlevel token
    parser.lexer.next_token();
    consume_new_line_token(parser);
    // current testuite path ex: "./testsuite.ll" or "./testsuite/testsuite.ll"
    let current_path = parser.ctx.path.clone();
    let parent_path = get_parent(&current_path).clone().join("../testcases/");
    loop {
        let token_type = parser.lexer.peek_token();
        match token_type {
            TokenType::IDENTIFIER(string) => {
                parser
                    .ctx
                    .set_path(&{ parent_path.join(string.to_owned() + ".ll") });
                let source_code = read_file_to_string(&parser.ctx.path);
                let tc_lexer = source_code_to_lexer(source_code, parser.ctx);
                //current testsuite lexer
                let current_lexer = parser.lexer.clone();
                // assign testcase lexer
                parser.set_lexer(tc_lexer);
                //println!("{:#?}", parser.lexer.tokens);
                let testcase = parse_tc(parser);
                testsuite.push_testcase(testcase);
                // reassign testsuite lexer
                parser.set_lexer(current_lexer);
                //cosume current testcase
                parser.lexer.next_token();
            }
            _ => break,
        }
        consume_new_line_token(parser);
    }
}

fn parse_prerequisite(testcase: &mut TestSuite, parser: &mut Parser) {
    todo!()
}
