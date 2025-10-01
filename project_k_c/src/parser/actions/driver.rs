use std::collections::HashMap;

use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::expression::{ExpKind, Literal};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::ast::teststep::TestStep;
use crate::class::WebDriverAction;
use crate::class::{Class, Method, WEB_DRIVER};
use crate::location::Span_Trait;
use crate::parser::errors::{EXPECT_EXPR, EXPECT_STRING_EXPR, VALID_URL, VALID_URL_SHCEME};
use crate::parser::translator_stack::TranslatorStack;
use crate::token::Token;
use manodae::error::ParseError;
use url::Url;

pub struct Driver {}

impl WebDriverAction for Driver {
    fn NAVIGATE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let navigate_token = _token_stack.pop().unwrap();
        let url = _tl_stack.pop().unwrap();
        let url_expr = if let TranslatorStack::Expression(expr) = &url {
            expr.clone()
        } else {
            let nt = navigate_token.make_dummy_token(&url.get_span());
            _errors.push(ParseError::new(nt, EXPECT_EXPR.to_string()));
            return;
        };

        if Primitives::String != url_expr.primitive {
            let nt = navigate_token.make_dummy_token(&url.get_span());
            _errors.push(ParseError::new(nt, EXPECT_STRING_EXPR.to_string()));
            return;
        }

        if let ExpKind::Lit(literal) = &url_expr.kind {
            if let Literal::String(url) = literal {
                match Url::parse(&url[..]) {
                    Ok(parsed_url) => {
                        if parsed_url.scheme() != "https" {
                            _errors.push(ParseError {
                                token: _token_stack.last().unwrap().clone(),
                                message: String::from(VALID_URL_SHCEME),
                                production_end: false,
                            })
                        }
                    }
                    Err(_) => {
                        let token = navigate_token.make_dummy_token(&url_expr.span);
                        _errors.push(ParseError::new(token, VALID_URL.to_string()));
                    }
                };
            }
        }

        let span = navigate_token.span.to(&url_expr.span);
        let arguments = HashMap::from([(URL_ARGKEY, Args::Expr(url_expr))]);

        let test_step = TestStep::new(
            span,
            Class::WEB_DRIVER,
            Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE),
            arguments,
        );

        _testcase.insert_teststep(TestcaseBody::TESTSTEP(test_step));
    }

    fn CLOSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let close_token = _token_stack.pop().unwrap();
        let teststep = TestStep::new(
            close_token.span,
            Class::WEB_DRIVER,
            Method::WEB_DRIVER(WEB_DRIVER::CLOSE),
            HashMap::new(),
        );
        _tl_stack.push(TranslatorStack::TestStep(teststep));
    }

    fn FIND_ELEMENT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_CURRENT_URL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let url_token = _token_stack.pop().unwrap();
        let _current_token = _token_stack.pop().unwrap();
        let get_token = _token_stack.pop().unwrap();
        let span = get_token.span.to(&url_token.span);
        let getter = Getter {
            span,
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };

        _tl_stack.push(TranslatorStack::Getter(getter));
    }

    fn GET_PAGE_SOURCE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    fn GET_TITLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let title_token = _token_stack.pop().unwrap();
        let get_token = _token_stack.pop().unwrap();
        let span = get_token.span.to(&title_token.span);
        let teststep = Getter {
            span,
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_TITLE),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };

        _tl_stack.push(TranslatorStack::Getter(teststep));
    }
    fn GET_WINDOW_HANDLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
}
