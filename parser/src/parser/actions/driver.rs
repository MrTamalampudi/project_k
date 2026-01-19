use std::collections::HashMap;

use crate::a_types;
use crate::parser::errors::{EXPECT_STRING_EXPR, VALID_URL, VALID_URL_SHCEME};
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::{TLVec, TranslatorStack};
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
use url::Url;
pub struct Driver;

impl WebDriverAction for Driver {
    a_types!();
    #[pop_token(navigate_token)]
    #[pop_expr(url_expr)]
    fn NAVIGATE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) -> () {
        if Primitives::String != url_expr.primitive {
            _errors.push_error(&url_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        if let ExpKind::Lit(literal) = &url_expr.kind {
            if let Literal::String(url) = literal {
                match Url::parse(&url[..]) {
                    Ok(parsed_url) => {
                        if parsed_url.scheme() != "https" {
                            _errors.push_error(&url_expr.span, String::from(VALID_URL_SHCEME))
                        }
                    }
                    Err(_) => _errors.push_error(&url_expr.span, String::from(VALID_URL)),
                };
            }
        }

        let span = navigate_token.1.start..url_expr.span.end;
        let arguments = HashMap::from([(URL_ARGKEY, Args::Expr(url_expr))]);

        let test_step = Action::new(span, Method::WEB_DRIVER(WEB_DRIVER::NAVIGATE), arguments);

        _tl_stack.push_step(Teststep::Action(test_step));
    }

    #[pop_token(close_token)]
    fn CLOSE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError>,
    ) -> () {
        let action = Action::new(
            close_token.1,
            Method::WEB_DRIVER(WEB_DRIVER::CLOSE),
            HashMap::new(),
        );
        _tl_stack.push_step(Teststep::Action(action));
    }
}
