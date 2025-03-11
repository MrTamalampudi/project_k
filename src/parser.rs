use crate::{
    actions::{Action, ActionOption},
    ast::{TestStep, Testcase},
    keywords::TokenType,
    lexer::Lexer,
};

pub fn parse_test_case(lexer: &mut Lexer) -> Testcase {
    let mut testcase: Testcase = Testcase::init();
    parse_top_level_items(lexer, &mut testcase);
    testcase
}

pub fn parse_top_level_items(lexer: &mut Lexer, testcase: &mut Testcase) {
    while let token = lexer.peek_token() {
        match token {
            TokenType::TESTSTEPS => parse_test_step(lexer, testcase),
            TokenType::EOF => break,
            _ => break,
        }
    }
}

pub fn parse_test_step(lexer: &mut Lexer, testcase: &mut Testcase) {
    let _ = lexer.next_token(); //consume "TestSteps" token
    while let token = lexer.peek_token() {
        match token {
            TokenType::ACTION_NAVIGATE => parse_navigate_action(lexer, testcase),
            TokenType::ACTION_CLICK => parse_click_action(lexer, testcase),
            TokenType::ACTION_BACK => parse_back_action(lexer, testcase),
            TokenType::ACTION_FORWARD => parse_forward_action(lexer, testcase),
            _ => break,
        }
    }
}

fn parse_back_action(lexer: &mut Lexer, testcase: &mut Testcase) {
    let back_token = lexer.next_token();
    let teststep: TestStep = TestStep::new(
        back_token.get_start_location(),
        back_token.get_start_location(),
        Action::BACK,
        ActionOption::NONE,
        vec![],
    );
    testcase.insert_teststep(teststep);
}

fn parse_forward_action(lexer: &mut Lexer, testcase: &mut Testcase) {
    let forward_token = lexer.next_token();
    let teststep: TestStep = TestStep::new(
        forward_token.get_start_location(),
        forward_token.get_start_location(),
        Action::FORWARD,
        ActionOption::NONE,
        vec![],
    );
    testcase.insert_teststep(teststep);
}

fn parse_navigate_action(lexer: &mut Lexer, testcase: &mut Testcase) {
    let navigate_token = lexer.next_token();
    //check if next token is String
    let url = match lexer.peek_token() {
        TokenType::STRING(url) => url.clone(),
        x @ _ => panic!("Expected String but got {:#?}", x),
    };
    //consume string token
    let url_token = lexer.next_token();

    testcase.insert_teststep(TestStep::new(
        navigate_token.get_start_location(),
        url_token.get_start_location(),
        Action::NAVIGATE,
        ActionOption::NONE,
        vec![url],
    ));
}

fn parse_click_action(lexer: &mut Lexer, testcase: &mut Testcase) {
    let click_token = lexer.next_token();
    //check if next token is string
    let locator = match lexer.peek_token() {
        TokenType::STRING(locator) => locator.clone(),
        x @ _ => panic!("Expected String but got {:#?}", x),
    };

    //consume string token
    let locator_token = lexer.next_token();

    testcase.insert_teststep(TestStep::new(
        click_token.get_start_location(),
        locator_token.get_start_location(),
        Action::CLICK,
        ActionOption::NONE,
        vec![locator],
    ));
}
