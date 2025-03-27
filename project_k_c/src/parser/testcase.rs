use super::Parser;
use crate::actions::{Action, ActionOption};
use crate::ast::{TestCase, TestStep};
use crate::enums::{Browser, Capabilities, CapabilityValue};
use crate::keywords::TokenType;
use crate::{read_file_to_string, source_code_to_tokens};
use crate::{source_code_to_lexer, Lexer};
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse_test_case(parser: &mut Parser) -> Rc<RefCell<TestCase>> {
    let mut testcase: TestCase = TestCase::init();
    parser.lexer.next_token(); //consume "#TESTCASE" token
    parse_top_level_items(&mut testcase, parser);
    parser.ctx.program.push_testcase(&testcase)
}

fn parse_top_level_items(testcase: &mut TestCase, parser: &mut Parser) {
    while let token = parser.lexer.peek_token() {
        match token {
            TokenType::TESTSTEPS => parse_test_step(testcase, parser),
            TokenType::PREREQUISITE => parse_prerequisite(testcase, parser),
            TokenType::CAPABILITIES => parse_capbilities(testcase, parser),
            TokenType::NEW_LINE => {
                parser.lexer.next_token();
            }
            TokenType::EOF => break,
            _ => break,
        }
    }
}

//@todo write better logic
fn parse_capbilities(testcase: &mut TestCase, parser: &mut Parser) {
    parser.lexer.next_token(); // consume capability token
    loop {
        let token = parser.lexer.peek_token().clone();
        let capability = match token {
            TokenType::IDENTIFIER(string) => {
                let token = parser.lexer.next_token(); //consume capabilitity key
                if !Capabilities::is_capability_key_valid(&string) {
                    parser.ctx.errors.insert_error(
                        format!("Expected a valid capability key"),
                        token.get_start_location(),
                        token.get_end_location(),
                        token.get_source_path(),
                    );
                    //consume till new line token
                    consume_till_new_line_token(parser.lexer);

                    continue;
                }
                let capbility = Capabilities::from_string(string.as_str());

                match parser.lexer.peek_token() {
                    TokenType::ASSIGN_OP => {
                        parser.lexer.next_token();
                    } // consume assign token
                    x @ _ => panic!("Expected Assign token got {}", x),
                }

                capbility
            }
            _ => break,
        };

        match capability {
            Capabilities::BROWSER => parse_browser_capability(testcase, parser),
            Capabilities::DRIVERURL => parse_driver_url_capability(testcase, parser),
            Capabilities::NONE => break,
        }
    }
}

fn parse_driver_url_capability(testcase: &mut TestCase, parser: &mut Parser) {
    let token = parser.lexer.next_token();
    let url = match token.get_token_type() {
        //both chrome and "chrome" with and without quotes are considered valid
        TokenType::STRING(string) => CapabilityValue::STRING(string),
        _ => {
            parser.ctx.errors.insert_error(
                "Expected a valid browser".to_string(),
                token.get_start_location(),
                token.get_end_location(),
                token.get_source_path(),
            );
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
            parser.ctx.errors.insert_error(
                "Expected a valid browser".to_string(),
                token.get_start_location(),
                token.get_end_location(),
                token.get_source_path(),
            );
            return;
        }
    };
    let capability_value = browser.match_capability_value();
    testcase.insert_capability(&String::from("browser"), &capability_value);
}

fn consume_till_new_line_token(lexer: &mut Lexer) {
    loop {
        let token = lexer.next_token();
        let token_type = token.get_token_type();
        match token_type {
            TokenType::NEW_LINE => break,
            _ => continue,
        }
    }
}

fn parse_test_step(testcase: &mut TestCase, parser: &mut Parser) {
    let _ = parser.lexer.next_token(); //consume "TestSteps" token
    while let token = parser.lexer.peek_token() {
        match token {
            TokenType::ACTION_NAVIGATE => parse_navigate_action(testcase, parser),
            TokenType::ACTION_CLICK => parse_click_action(testcase, parser),
            TokenType::ACTION_BACK => parse_back_action(testcase, parser),
            TokenType::ACTION_FORWARD => parse_forward_action(testcase, parser),
            TokenType::IDENTIFIER(ident) => parse_variable_initalization(testcase, parser),
            TokenType::NEW_LINE => {
                parser.lexer.next_token(); // consume NEW_LINE token
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

fn parse_prerequisite(testcase: &mut TestCase, parser: &mut Parser) {
    parser.lexer.next_token(); //consume PREREQUISITE token
    loop {
        let token = parser.lexer.peek_token();
        match token {
            TokenType::IDENTIFIER(string) => {
                let prerequisite_path = parser.ctx.parent_path.to_owned() + string.as_str() + ".ll";
                let path = parser.ctx.path.clone();
                parser.ctx.path = prerequisite_path; // assign prerequisite path
                let source_code = read_file_to_string(&parser.ctx.path);
                let prerequiste_lexer = source_code_to_lexer(source_code, parser.ctx);
                let current_lexer = parser.lexer.clone(); //current lexer
                parser.set_lexer(prerequiste_lexer); // assign preruqisite lexer

                //println!("{:#?}", parser.lexer.tokens);
                let prerequisite_testcase = parse_test_case(parser);
                parser.ctx.path = path; //reassign current path
                testcase.insert_prerequisite(prerequisite_testcase);
                parser.set_lexer(current_lexer);

                parser.lexer.next_token(); // consume current token
            }
            _ => break,
        }
    }
}
