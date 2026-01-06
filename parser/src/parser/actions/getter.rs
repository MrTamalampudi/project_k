use std::collections::HashMap;

use ast::{
    expression::{ExpKind, Expr, Literal},
    ArgKeys::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY},
    Getter as G, LocatorStrategy, Primitives, TestCase,
};
use class::{GetterAction, Method, GETTER};
use macros::{pop_expr, pop_token};
use manodae::error::ParseError;

use crate::{
    a_types,
    parser::{
        actions::shared::Shared,
        errors::EXPECT_STRING_EXPR,
        errorss::ActionError,
        translator_stack::{TLVec, TranslatorStack},
    },
    token::Token,
};

pub struct Getter;

impl Getter {
    fn is_common(
        start_token: Token,
        end_token: Token,
        expr: Expr,
        method: GETTER,
        e: &mut Vec<ParseError<Token>>,
        t: &mut Vec<TranslatorStack>,
    ) {
        let span = start_token.span.to(&end_token.span);
        let locator_arg = match Shared::get_locator_arg(&expr) {
            Ok(arg) => arg,
            Err(err) => {
                e.push_error(&start_token, &expr.span, err.clone());
                return;
            }
        };
        let getter = G {
            span,
            method: Method::GETTER(method),
            arguments: HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::Boolean,
        };
        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::Boolean,
        };
        t.push_expr(expr);
    }

    fn get_common(
        get_token: Token,
        expr: Expr,
        locator_expr: Expr,
        method: GETTER,
        e: &mut Vec<ParseError<Token>>,
        t: &mut Vec<TranslatorStack>,
    ) {
        if Primitives::String != expr.primitive {
            e.push_error(&get_token, &expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        if Primitives::String != locator_expr.primitive {
            e.push_error(
                &get_token,
                &locator_expr.span,
                EXPECT_STRING_EXPR.to_string(),
            );
            return;
        }

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &locator_expr.kind {
            Args::Locator(LocatorStrategy::parse(&locator))
        } else {
            Args::Expr(locator_expr.clone())
        };

        let target = if let ExpKind::Lit(Literal::String(target)) = expr.kind {
            Args::String(target)
        } else {
            Args::Expr(expr)
        };

        let span = get_token.span.to(&locator_expr.span);
        let getter = G {
            span,
            method: Method::GETTER(method),
            arguments: HashMap::from([(ATTRIBUTE_ARGKEY, target), (LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        t.push_expr(expr);
    }
}

impl GetterAction for Getter {
    a_types!();
    //get attribute expression from element expression
    #[pop_token(_element, _from, _attribute, get_token)]
    #[pop_expr(locator_expr, attribute_expr)]
    fn GET_ATTRIBUTE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        Getter::get_common(
            get_token,
            attribute_expr,
            locator_expr,
            GETTER::GET_ATTRIBUTE,
            _errors,
            _tl_stack,
        );
    }

    //action: is element expression displayed
    #[pop_token(displayed, _element, is)]
    #[pop_expr(expr)]
    fn IS_DISPLAYED(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
        Getter::is_common(
            is,
            displayed,
            expr,
            GETTER::IS_DISPLAYED,
            _errors,
            _tl_stack,
        );
    }

    #[pop_token(enabled, _element, is)]
    #[pop_expr(expr)]
    fn IS_ENABLED(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        Getter::is_common(is, enabled, expr, GETTER::IS_ENABLED, _errors, _tl_stack);
    }
    #[pop_token(selected, _element, is)]
    #[pop_expr(expr)]
    fn IS_SELECTED(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        Getter::is_common(is, selected, expr, GETTER::IS_SELECTED, _errors, _tl_stack);
    }

    #[pop_token(url_token, _current_token, get_token)]
    fn GET_CURRENT_URL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let span = get_token.span.to(&url_token.span);
        let getter = G {
            span,
            method: Method::GETTER(GETTER::GET_CURRENT_URL),
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
    #[pop_token(title_token, get_token)]
    fn GET_TITLE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) -> () {
        let span = get_token.span.to(&title_token.span);
        let getter = G {
            span,
            method: Method::GETTER(GETTER::GET_TITLE),
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

    //Get Text From Element Expression
    #[pop_token(_element, _from, _text, get)]
    #[pop_expr(locator_expr)]
    fn GET_TEXT(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        if Primitives::String != locator_expr.primitive {
            _errors.push_error(&get, &locator_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &locator_expr.kind {
            Args::Locator(LocatorStrategy::parse(&locator))
        } else {
            Args::Expr(locator_expr.clone())
        };

        let span = get.span.to(&locator_expr.span);
        let getter = G {
            span,
            method: Method::GETTER(GETTER::GET_TEXT),
            arguments: HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }

    //Get Css Value Expression From Element Expression
    #[pop_token(_element, _from, _value, _css, get)]
    #[pop_expr(locator_expr, css_expr)]
    fn GET_CSS_VALUE(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        if Primitives::String != css_expr.primitive {
            _errors.push_error(&get, &locator_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        if Primitives::String != locator_expr.primitive {
            _errors.push_error(&get, &locator_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &locator_expr.kind {
            Args::Locator(LocatorStrategy::parse(&locator))
        } else {
            Args::Expr(locator_expr.clone())
        };

        let css = if let ExpKind::Lit(Literal::String(css)) = css_expr.kind {
            Args::String(css)
        } else {
            Args::Expr(css_expr)
        };

        let span = get.span.to(&locator_expr.span);
        let getter = G {
            span,
            method: Method::GETTER(GETTER::GET_CSS_VALUE),
            arguments: HashMap::from([(ATTRIBUTE_ARGKEY, css), (LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }

    //Get Tag Name From Element Expression
    #[pop_token(_element, _from, _tag, _name, get)]
    #[pop_expr(locator_expr)]
    fn GET_TAG_NAME(
        _testcase: &mut Self::AST,
        _token_stack: &mut Vec<Self::Token>,
        _tl_stack: &mut Vec<Self::TranslatorStack>,
        _errors: &mut Vec<Self::Error<Self::Token>>,
    ) {
        if Primitives::String != locator_expr.primitive {
            _errors.push_error(&get, &locator_expr.span, EXPECT_STRING_EXPR.to_string());
            return;
        }

        let locator_arg = if let ExpKind::Lit(Literal::String(locator)) = &locator_expr.kind {
            Args::Locator(LocatorStrategy::parse(&locator))
        } else {
            Args::Expr(locator_expr.clone())
        };

        let span = get.span.to(&locator_expr.span);
        let getter = G {
            span,
            method: Method::GETTER(GETTER::GET_TAG_NAME),
            arguments: HashMap::from([(LOCATOR_ARGKEY, locator_arg)]),
            returns: Primitives::String,
        };

        let expr = Expr {
            span,
            kind: ExpKind::Getter(getter),
            primitive: Primitives::String,
        };

        _tl_stack.push_expr(expr);
    }
}
