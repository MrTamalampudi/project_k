use crate::{
    actions::{Action, ActionOption},
    ast::{TestStep, Testcase},
    compile,
    keywords::TokenType,
    lexer::{self, Lexer},
    read_file_to_string, source_code_to_lexer, CompilationContext, ExecutionType,
};

pub fn parse_test_case(lexer: &mut Lexer, compilation_context: &CompilationContext) -> Testcase {
    let mut testcase: Testcase = Testcase::init();
    lexer.next_token(); //consume TESTCASE token
    parse_top_level_items(lexer, &mut testcase, compilation_context);
    testcase
}

pub fn parse_top_level_items(
    lexer: &mut Lexer,
    testcase: &mut Testcase,
    compilation_context: &CompilationContext,
) {
    while let token = lexer.peek_token() {
        match token {
            TokenType::TESTSTEPS => parse_test_step(lexer, testcase),
            TokenType::PREREQUISITE => parse_prerequisite(lexer, &compilation_context, testcase),
            TokenType::EOF => break,
            _ => break,
        }
    }
}

fn parse_prerequisite(
    lexer: &mut Lexer,
    compilation_context: &CompilationContext,
    testcase: &mut Testcase,
) {
    lexer.next_token(); //consume PREREQUISITE token
    loop {
        let token = lexer.peek_token();
        match token {
            TokenType::IDENTIFIER(string) => {
                let prerequisite_path =
                    compilation_context.project_root.to_owned() + "/" + string.as_str() + ".ll";
                let source_code = read_file_to_string(&prerequisite_path);
                let mut prerequiste_lexer = source_code_to_lexer(source_code);
                let prerequisite_testcase =
                    parse_test_case(&mut prerequiste_lexer, &compilation_context);
                testcase.insert_prerequisite(prerequisite_testcase);
                lexer.next_token(); // consume current token
            }
            _ => break,
        }
    }
}

fn parse_test_step(lexer: &mut Lexer, testcase: &mut Testcase) {
    let _ = lexer.next_token(); //consume "TestSteps" token
    while let token = lexer.peek_token() {
        match token {
            TokenType::ACTION_NAVIGATE => parse_navigate_action(lexer, testcase),
            TokenType::ACTION_CLICK => parse_click_action(lexer, testcase),
            TokenType::ACTION_BACK => parse_back_action(lexer, testcase),
            TokenType::ACTION_FORWARD => parse_forward_action(lexer, testcase),
            TokenType::IDENTIFIER(ident) => parse_variable_initalization(lexer, testcase),
            _ => break,
        }
    }
}

fn parse_variable_initalization(lexer: &mut Lexer, testcase: &mut Testcase) {
    let identifier = match lexer.next_token().get_token_type() {
        TokenType::IDENTIFIER(ident) => ident,
        x @ _ => panic!("Expected String token got {x}"),
    };
    match lexer.peek_token() {
        TokenType::ASSIGN_OP => {}
        x @ _ => panic!("Expected Assign token got {x}"),
    };
    lexer.next_token(); // consume Assign token
    let value = lexer.next_token().get_token_type().match_identifier_value();
    testcase.insert_variable(&identifier, &value);
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
