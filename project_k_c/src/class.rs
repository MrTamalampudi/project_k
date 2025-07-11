#![allow(non_camel_case_types, non_snake_case)]

use crate::ast::AST;
use crate::token::Token;
use slr_parser::error::ParseError;
use std::future::Future;

macro_rules! ifdef {
    ([$($_:tt)+] { $($then:tt)* } $(else { $($_else:tt)* })?) => {
        $($then)*
    };
    ([] { $($_then:tt)* } $(else { $($else:tt)* })?) => {
        $($($else)*)?
    };
}

macro_rules! class_macro {
    (
        $(
            {
                action:$action:ident,
                engine:$engine:ident,
                $class:ident {
                    $(
                        $method:ident$(
                            {
                                $(Action_return_type: $action_return_type:ident,)?
                                $(Engine_return_type: $engine_return_type:ident)?
                            }
                        )?
                    ),+
                }
            }
        ),+
    ) => {
        #[derive(Debug, Clone)]
        pub enum Class {
            $($class),+
        }

        $(
            #[derive(Debug, Clone)]
            pub enum $class {
                $($method),+
            }
        )+

        #[derive(Debug, Clone)]
        pub enum Method {
            $($class($class)),+
        }

        $(
            pub trait $engine{
                $(
                    ifdef! {[$($($engine_return_type)?)?]
                        {fn $method(&self) -> impl Future<Output = $($($engine_return_type)?)?>;}
                        else
                        {fn $method(&self) -> impl Future<Output = ()>;}
                    }
                )+
            }
        )+

        $(
            pub trait $action{
                $(
                    ifdef! {[$($($action_return_type)?)?]
                        {fn $method(
                            testcase: &mut TestCase,
                            token_stack: &mut Vec<Token>,
                            errors: &mut Vec<ParseError<Token>>
                        ) -> $($($action_return_type)?)?;}
                        else
                        {fn $method(
                            ast: &mut Vec<AST>,
                            token_stack: &mut Vec<Token>,
                            errors: &mut Vec<ParseError<Token>>
                        );}
                    }
                )+
            }
        )+
    };
}

class_macro!(
    {
        action: ELEMENT_ACTION,
        engine: ELENENT_ENGINE,
        ELEMENT {
            CLEAR,
            CLICK,
            GET_ACCESSBILE_NAME,
            GET_ARIA_ROLE,
            GET_ATTRIBUTE,
            GET_CSS_VALUE,
            GET_DOM_PROPERTY,
            GET_LOCATION,
            GET_SIZE,
            GET_TAG_NAME,
            GET_TEXT,
            IS_DISPLAYED,
            IS_ENABLED,
            IS_SELECTED,
            SENDKEYS,
            SUBMIT
        }
    },
    {
        action: WINDOW_ACTION,
        engine: WINDOW_ENGINE,
        WINDOW {
            FULL_SCREEN,
            GET_POSITION,
            GET_SIZE,
            MAXIMIZE,
            MINIMIZE
        }
    },
    {
        action: WEB_DRIVER_ACTION,
        engine: WEB_DRIVER_ENGINE,
        WEB_DRIVER {
            GET_TITLE {
                Engine_return_type: String
            },
            GET_CURRENT_URL,
            GET_PAGE_SOURCE,
            GET_WINDOW_HANDLE,
            CLOSE,
            FIND_ELEMENT,
            NAVIGATE
        }
    },
    {
        action: ALERT_ACTION,
        engine: ALERT_ENGINE,
        ALERT {
            ACCEPT,
            DISMISS,
            GET_TEXT,
            SEND_KEYS
        }
    },
    {
        action: NAVIGATION_ACTION,
        engine: NAVIGATION_ENGINE,
        NAVIGATION {
            BACK,
            FORWARD,
            REFRESH
        }
    },
    {
        action: OPTIONS_ACTION,
        engine: OPTIONS_ENGINE,
        OPTIONS {
            ADD_COOKIE,
            DELETE_ALL_COOKIES,
            DELETE_COOKIE,
            GET_COOKIE
        }
    },
    {
        action: TARGET_LOCATOR_ACTION,
        engine: TARGET_LOCATOR_ENGINE,
        TARGET_LOCATOR {
            ACTIVE_ELEMENT,
            ALERT,
            FRAME,
            NEW_WINDOW,
            PARENT_FRAME,
            WINDOW
        }
    },
    {
        action: TIMEOUTS_ACTION,
        engine: TIMEOUTS_ENGINE,
        TIMEOUTS {
            GET_IMPLICIT_TIMEOUT,
            GET_PAGE_LOAD_TIMEOUT,
            SET_IMPLICIT_TIMEOUT,
            SET_PAGE_LOAD_TIMEOUT
        }
    },
    {
        action: HAS_DOWNLOADS_ACTION,
        engine: HAS_DOWNLOADS_ENGINE,
        HAS_DOWNLOADS {
            DELETE_DOWNLOADABLE_FILES,
            DOWNLOAD_FILE,
            GET_DOWNLOADABLE_FILES
        }
    }
);
