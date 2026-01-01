#![allow(non_camel_case_types, non_snake_case, unused_parens)]

use std::future::Future;

pub trait GetMethod {
    fn get_method(&self) -> Method;
}

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
                                        $(args: [$($engine_arg_ident:ident: $engine_arg_type:ty),*])?
                                        $(returns : $engine_returns:ty)?
                                    }
                                )?
                            }
                        )?
                    ),+
                }
            }
        ),+
    ) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub enum $class {
                $($method),+
            }
        )+

        #[derive(Debug, Clone, PartialEq)]
        pub enum Method {
            $($class($class)),+
        }

        $(
            pub trait $engine{
                type Step;
                type Error;
                $(
                    ifdef! {
                        [$($($($engine_returns)?)?)?,$($($($($engine_arg_ident),*)?)?)?]
                        {fn $method(&mut self,_step:&Self::Step,$($($($($engine_arg_ident:$engine_arg_type),*)?)?)?) -> impl Future<Output = Result<($($($($engine_returns)?)?)?),Self::Error>>;}
                        else
                        { ifdef! {
                            [$($($($engine_returns)?)?)?]
                            {fn $method(&mut self,_step:&Self::Step) -> impl Future<Output = (Result<$($($($engine_returns)?)?)?,Self::Error>)>;}
                            else
                            { ifdef! {
                                [$($($($($engine_arg_ident),*)?)?)?]
                                {fn $method(&mut self,_step:&Self::Step,$($($($($engine_arg_ident:$engine_arg_type),*)?)?)?) -> impl Future<Output = (Result<(),Self::Error>)>;}
                                else
                                {fn $method(&mut self,_step:&Self::Step) -> impl Future<Output = (Result<(),Self::Error>)>;}
                                }
                            }}
                        }
                    }
                )+
            }
        )+
        $(
            pub trait $action{
                type AST;
                type Token;
                type TranslatorStack;
                type Error<Token>;
                $(
                    ifdef! {[$($($action_returns)?)?]
                        {fn $method(
                            _testcase: &mut Self::AST,
                            _token_stack: &mut Vec<Self::Token>,
                            _tl_stack:&mut Vec<Self::TranslatorStack>,
                            _errors: &mut Vec<Self::Error<Self::Token>>
                        ) -> $($($action_returns)?)?;}
                        else
                        {fn $method(
                            _testcase: &mut Self::AST,
                            _token_stack: &mut Vec<Self::Token>,
                            _tl_stack:&mut Vec<Self::TranslatorStack>,
                            _errors: &mut Vec<Self::Error<Self::Token>>
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
                    returns: Option<String>
                }
            },
            // GET_ACCESSBILE_NAME,
            // GET_ARIA_ROLE,
            // GET_CSS_VALUE,
            // GET_DOM_PROPERTY,
            // GET_LOCATION,
            // GET_SIZE,
            // GET_TAG_NAME,
            // GET_TEXT,
            IS_DISPLAYED {
                engine:{
                    returns: Option<bool>
                }
            }
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
                    returns: Option<String>
                }
            },
            GET_CURRENT_URL {
                engine: {
                    returns: Option<String>
                }
            },
            GET_PAGE_SOURCE,
            GET_WINDOW_HANDLE,
            CLOSE,
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
            WAIT
            // GET_IMPLICIT_TIMEOUT,
            // GET_PAGE_LOAD_TIMEOUT,
            // SET_IMPLICIT_TIMEOUT,
            // SET_PAGE_LOAD_TIMEOUT
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
            VAR_DECLARATION,
            ASSERT
        }
    },
    {
        action:ArthimaticExpressionAction,
        engine:ArthimaticExpressionEngine,
        ARTHIMATICEXPRESSION {
            PLUS,
            MINUS,
            SP_MINUS,
            MULTIPLY,
            DIVISION,
            MODULUS
        }
    },
    {
        action:LiteralExpressionAction,
        engine:LiteralExpressionEngine,
        LITERALEXPRESSION {
            STRING,
            NUMBER,
            IDENT,
            BOOLEAN,
            ARRAY
        }
    },
    {
        action:UnaryExpressionAction,
        engine:UnaryExpressionEngine,
        UNARYEXPRESSION  {
            NEGATION,
            GROUPED
        }
    },
    {
        action:BinaryExpressionAction,
        engine:BinaryExpressionEngine,
        BINARYEXPRESSION {
            ADD,
            SUB,
            MUL,
            DIV,
            REM,
            AND,
            OR,
            EQ,
            LT,
            LE,
            NE,
            GE,
            GT
        }
    },
    {
        action:ControlFlowAction,
        engine:ControlFlowEngine,
        CONTROL_FLOW {
            IF,
            ELSE_IF,
            ELSE,
            WHILE,
            FOR
        }
    }
);
