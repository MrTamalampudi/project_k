use std::collections::HashMap;

use crate::a_types;
use crate::parser::errors::{EXPECT_STRING_EXPR, VALID_URL, VALID_URL_SHCEME};
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use ast::expression::{ExpKind, Literal};
use ast::Action;
use ast::ArgKeys::{Args, URL_ARGKEY};
use ast::Primitives;
use ast::TestCase;
use ast::Teststep;
use class::WebDriverAction;
use class::{Method, WEB_DRIVER};
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;
use span::SpanData;
use url::Url;

pub struct Driver;

impl WebDriverAction for Driver {
    a_types!();
    #[pop_token(navigate_token)]
    #[pop_expr(url_expr)]
    fn NAVIGATE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        if Primitives::String != url_expr.primitive {
            let nt = navigate_token.make_dummy_token(&url_expr.get_span());
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

        let test_step = Action::new(span, Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE), arguments);

        _tl_stack.push_step(Teststep::Action(test_step));
    }

    #[pop_token(close_token)]
    fn CLOSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let action = Action::new(
            close_token.span,
            Method::WEB_DRIVER(WEB_DRIVER::CLOSE),
            HashMap::new(),
        );
        _tl_stack.push_step(Teststep::Action(action));
    }
}
