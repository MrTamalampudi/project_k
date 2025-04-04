use super::errors::{collect_capability_key_error, collect_prerequisite_path_error};
use super::{consume_new_line_token, Parser};
use crate::actions::{Action, ActionOption};
use crate::ast::{TestCase, TestStep};
use crate::enums::{Browser, Capabilities, CapabilityValue};
use crate::keywords::TokenType;
use crate::lexer::Token;
use crate::utils::get_parent;
use crate::{read_file_to_string, source_code_to_tokens};
use crate::{source_code_to_lexer, Lexer};
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_testcase(parser: &mut Parser) -> Rc<RefCell<TestCase>> {
    let mut testcase: TestCase = TestCase::new();
    //consume "#TESTCASE" token
    parser.lexer.next_token();
    consume_new_line_token(parser);
    parse_top_level_items(&mut testcase, parser);
    parser.ctx.program.push_testcase(&testcase)
}

fn parse_top_level_items(testcase: &mut TestCase, parser: &mut Parser) {
    loop {
        let token = parser.lexer.peek_token();
        match token {
            TokenType::TESTSTEPS => parse_test_step(testcase, parser),
            TokenType::PREREQUISITE => parse_prerequisite(testcase, parser),
            TokenType::CAPABILITIES => parse_capbilities(testcase, parser),
            TokenType::NEW_LINE => consume_new_line_token(parser),
            TokenType::EOF => break,
            _ => break,
        }
    }
}

//@todo write better logic
//handle errors properly
fn parse_capbilities(testcase: &mut TestCase, parser: &mut Parser) {
    //cosnume capability highleveltoken
    parser.lexer.next_token();
    consume_new_line_token(parser);

    loop {
        let token = parser.lexer.peek_token().clone();
        let capability = match token {
            TokenType::IDENTIFIER(string) => {
                //consume capabilitity key
                let token = parser.lexer.next_token();
                if !Capabilities::is_capability_key_valid(&string) {
                    collect_capability_key_error(&token, parser);
                    continue;
                }
                Capabilities::from_string(string.as_str())
            }
            _ => break,
        };

        consume_operator_token(TokenType::ASSIGN_OP, parser);

        match capability {
            Capabilities::BROWSER => parse_browser_capability(testcase, parser),
            Capabilities::DRIVERURL => parse_driver_url_capability(testcase, parser),
            Capabilities::NONE => break,
        }

        consume_new_line_token(parser);
    }
}

fn consume_operator_token(token_type: TokenType, parser: &mut Parser) {
    let token = parser.lexer.next_token();
    if token.get_token_type() != token_type {
        parser.ctx.errors.insert_parsing_error(
            format!("Expected Assign token got {}", token.get_token_type()),
            &token,
        );
    }
}

fn parse_driver_url_capability(testcase: &mut TestCase, parser: &mut Parser) {
    let token = parser.lexer.next_token();
    let url = match token.get_token_type() {
        //both chrome and "chrome" with and without quotes are considered valid
        TokenType::STRING(string) => CapabilityValue::STRING(string),
        _ => {
            parser
                .ctx
                .errors
                .insert_parsing_error("Expected a valid browser".to_string(), &token);
            return;
        }
    };
    testcase.insert_capability(&Capabilities::DRIVERURL.to_string().to_string(), &url);
}

fn parse_browser_capability(testcase: &mut TestCase, parser: &mut Parser) {
    let token = parser.lexer.next_token();
    let browser = match token.get_token_type() {
        //both chrome and "chrome" with and without quotes are considered valid
        TokenType::IDENTIFIER(string) => Browser::from_string(string.as_str()),
        TokenType::STRING(string) => Browser::from_string(string.as_str()),
        _ => {
            parser
                .ctx
                .errors
                .insert_parsing_error("Expected a valid browser".to_string(), &token);
            return;
        }
    };
    let capability_value = browser.match_capability_value();
    testcase.insert_capability(&String::from("browser"), &capability_value);
}

fn parse_test_step(testcase: &mut TestCase, parser: &mut Parser) {
    let _ = parser.lexer.next_token(); //consume "TestSteps" token
    loop {
        let token = parser.lexer.peek_token();
        match token {
            TokenType::ACTION_NAVIGATE => parse_navigate_action(testcase, parser),
            TokenType::ACTION_CLICK => parse_click_action(testcase, parser),
            TokenType::ACTION_BACK => parse_back_action(testcase, parser),
            TokenType::ACTION_FORWARD => parse_forward_action(testcase, parser),
            TokenType::IDENTIFIER(ident) => parse_variable_initalization(testcase, parser),
            TokenType::NEW_LINE => {
                // consume NEW_LINE token
                parser.lexer.next_token();
            }
            _ => break,
        }
    }
}

fn parse_variable_initalization(testcase: &mut TestCase, parser: &mut Parser) {
    let identifier = match parser.lexer.next_token().get_token_type() {
        TokenType::IDENTIFIER(ident) => ident,
        x @ _ => panic!("Expected String token got {x}"),
    };
    match parser.lexer.peek_token() {
        TokenType::ASSIGN_OP => {}
        x @ _ => panic!("Expected Assign token got {x}"),
    };
    parser.lexer.next_token(); // consume Assign token
    let value = parser
        .lexer
        .next_token()
        .get_token_type()
        .match_identifier_value();
    testcase.insert_variable(&identifier, &value);
}

fn parse_back_action(testcase: &mut TestCase, parser: &mut Parser) {
    let back_token = parser.lexer.next_token();
    let teststep: TestStep = TestStep::new(
        back_token.get_start_location(),
        back_token.get_start_location(),
        Action::BACK,
        ActionOption::NONE,
        vec![],
    );
    testcase.insert_teststep(teststep);
}

fn parse_forward_action(testcase: &mut TestCase, parser: &mut Parser) {
    let forward_token = parser.lexer.next_token();
    let teststep: TestStep = TestStep::new(
        forward_token.get_start_location(),
        forward_token.get_start_location(),
        Action::FORWARD,
        ActionOption::NONE,
        vec![],
    );
    testcase.insert_teststep(teststep);
}

fn parse_navigate_action(testcase: &mut TestCase, parser: &mut Parser) {
    let navigate_token = parser.lexer.next_token();
    //check if next token is String
    let url = match parser.lexer.peek_token() {
        TokenType::STRING(url) => url.clone(),
        x @ _ => panic!("Expected String but got {:#?}", x),
    };
    //consume string token
    let url_token = parser.lexer.next_token();

    testcase.insert_teststep(TestStep::new(
        navigate_token.get_start_location(),
        url_token.get_start_location(),
        Action::NAVIGATE,
        ActionOption::NONE,
        vec![url],
    ));
}

fn parse_click_action(testcase: &mut TestCase, parser: &mut Parser) {
    let click_token = parser.lexer.next_token();
    //check if next token is string
    let locator = match parser.lexer.peek_token() {
        TokenType::STRING(locator) => locator.clone(),
        x @ _ => panic!("Expected String but got {:#?}", x),
    };

    //consume string token
    let locator_token = parser.lexer.next_token();

    testcase.insert_teststep(TestStep::new(
        click_token.get_start_location(),
        locator_token.get_start_location(),
        Action::CLICK,
        ActionOption::NONE,
        vec![locator],
    ));
}

//todo solve circular prerequisite dependency
fn parse_prerequisite(testcase: &mut TestCase, parser: &mut Parser) {
    //consume PREREQUISITE token
    parser.lexer.next_token();
    consume_new_line_token(parser);
    loop {
        let token = parser.lexer.peek_token();
        match token {
            TokenType::IDENTIFIER(string) => {
                let path = parser.ctx.path.clone();
                // assign prerequisite path
                let prerequisite_path =
                    parser.ctx.get_parent_path().join(string.to_owned() + ".ll");

                if !prerequisite_path.exists() {
                    collect_prerequisite_path_error(parser);
                    continue;
                }

                parser.ctx.set_path(&prerequisite_path);
                let source_code = read_file_to_string(&parser.ctx.path);
                let prerequiste_lexer = source_code_to_lexer(source_code, parser.ctx);
                //current lexer
                let current_lexer = parser.lexer.clone();
                // assign preruqisite lexer
                parser.set_lexer(prerequiste_lexer);
                //println!("{:#?}", parser.lexer.tokens);
                let prerequisite_testcase = parse_testcase(parser);
                parser.ctx.path = path; //reassign current path
                testcase.insert_prerequisite(prerequisite_testcase);
                parser.set_lexer(current_lexer);
                // consume current token
                parser.lexer.next_token();
            }
            TokenType::NEW_LINE => consume_new_line_token(parser),
            _ => break,
        }
    }
}
