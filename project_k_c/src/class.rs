#![allow(non_camel_case_types, non_snake_case, unused_parens)]

use crate::ast::testcase::TestCase;
use crate::ast::testcase_body::TestcaseBody;
use crate::parser::translator_stack::TranslatorStack;
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
                                $(
                                    action: {
                                        returns : $action_returns:ident
                                    }
                                )?
                                $(
                                    engine : {
                                        $(returns : $engine_returns:ident)?
                                    }
                                )?
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
                    ifdef! {
                        [$($($($engine_returns)?)?)?]
                        {fn $method(&self,_step:&TestcaseBody) -> impl Future<Output = ($($($($engine_returns)?)?)?)>;}
                        else
                        {fn $method(&self,_step:&TestcaseBody) -> impl Future<Output = ()>;}
                    }
                )+
            }
        )+

        $(
            pub trait $action{
                $(
                    ifdef! {[$($($action_returns)?)?]
                        {fn $method(
                            _testcase: &mut TestCase,
                            _token_stack: &mut Vec<Token>,
                            _tl_stack:&mut Vec<TranslatorStack>,
                            _errors: &mut Vec<ParseError<Token>>
                        ) -> $($($action_returns)?)?;}
                        else
                        {fn $method(
                            _testcase: &mut TestCase,
                            _token_stack: &mut Vec<Token>,
                            _tl_stack:&mut Vec<TranslatorStack>,
                            _errors: &mut Vec<ParseError<Token>>
                        );}
                    }
                )+
            }
        )+
    };
}

class_macro!(
    {
        action: ElementAction,
        engine: ElementEngine,
        ELEMENT {
            CLEAR,
            CLICK,
            SENDKEYS,
            SUBMIT,
            GET_ATTRIBUTE {
                engine:{
                    returns: String
                }
            }
            // GET_ACCESSBILE_NAME,
            // GET_ARIA_ROLE,
            // GET_CSS_VALUE,
            // GET_DOM_PROPERTY,
            // GET_LOCATION,
            // GET_SIZE,
            // GET_TAG_NAME,
            // GET_TEXT,
            // IS_DISPLAYED,
            // IS_ENABLED,
            // IS_SELECTED
        }
    },
    {
        action: WindowAction,
        engine: WindowEngine,
        WINDOW {
            FULL_SCREEN,
            GET_POSITION,
            GET_SIZE,
            MAXIMIZE,
            MINIMIZE
        }
    },
    {
        action: WebDriverAction,
        engine: WebDriverEngine,
        WEB_DRIVER {
            GET_TITLE {
                engine: {
                    returns: String
                }
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
        action: AlertAction,
        engine: AlertEngine,
        ALERT {
            ACCEPT,
            DISMISS,
            GET_TEXT,
            SEND_KEYS
        }
    },
    {
        action: NavigationAction,
        engine: NavigationEngine,
        NAVIGATION {
            BACK,
            FORWARD,
            REFRESH
        }
    },
    {
        action: OptionsAction,
        engine: OptionsEngine,
        OPTIONS {
            ADD_COOKIE,
            DELETE_ALL_COOKIES,
            DELETE_COOKIE,
            GET_COOKIE
        }
    },
    {
        action: TargetLocatorAction,
        engine: TargetLocatorEngine,
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
        action: TimeoutsAction,
        engine: TimeoutsEngine,
        TIMEOUTS {
            GET_IMPLICIT_TIMEOUT,
            GET_PAGE_LOAD_TIMEOUT,
            SET_IMPLICIT_TIMEOUT,
            SET_PAGE_LOAD_TIMEOUT
        }
    },
    {
        action: HasDownloadsAction,
        engine: HasDownloadsEngine,
        HAS_DOWNLOADS {
            DELETE_DOWNLOADABLE_FILES,
            DOWNLOAD_FILE,
            GET_DOWNLOADABLE_FILES
        }
    },
    {
        action:CustomAction,
        engine:CustomEngine,
        CUSTOM {
            VAR_DECLARATION
        }
    }
);
