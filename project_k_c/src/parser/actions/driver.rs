use std::collections::HashMap;

use crate::ast::action::Action;
use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::expression::{ExpKind, Expr, Literal};
use crate::ast::getter::Getter;
use crate::ast::primitives::Primitives;
use crate::ast::testcase::TestCase;
use crate::ast::teststep::Teststep;
use crate::class::WebDriverAction;
use crate::class::{Method, WEB_DRIVER};
use crate::parser::errors::{EXPECT_EXPR, EXPECT_STRING_EXPR, VALID_URL, VALID_URL_SHCEME};
use crate::parser::translator_stack::{TLVec, TranslatorStack};
use crate::token::Token;
use macros::pop_token;
use manodae::error::ParseError;
use span::SpanData;
use url::Url;

pub struct Driver;

impl WebDriverAction for Driver {
    #[pop_token(navigate_token)]
    fn NAVIGATE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
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

    #[pop_token(url_token, _current_token, get_token)]
    fn GET_CURRENT_URL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let span = get_token.span.to(&url_token.span);
        let getter = Getter {
            span,
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }

    fn GET_PAGE_SOURCE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }

    #[pop_token(title_token, get_token)]
    fn GET_TITLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let span = get_token.span.to(&title_token.span);
        let getter = Getter {
            span,
            method: Method::WEB_DRIVER(WEB_DRIVER::GET_TITLE),
            arguments: HashMap::new(),
            returns: Primitives::String,
        };
        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }
    fn GET_WINDOW_HANDLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
    }
}
