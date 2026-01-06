#![allow(non_camel_case_types, non_snake_case, unused_parens)]

pub trait GetMethod {
    fn get_method(&self) -> Method;
}

macro_rules! class_macro {
    (
        $(
            {
                action:$action:ident,
                $class:ident {
                    $(
                        $method:ident
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
            pub trait $action{
                type AST;
                type Token;
                type TranslatorStack;
                type Error<Token>;
                $(
                    fn $method(
                        _testcase: &mut Self::AST,
                        _token_stack: &mut Vec<Self::Token>,
                        _tl_stack:&mut Vec<Self::TranslatorStack>,
                        _errors: &mut Vec<Self::Error<Self::Token>>
                    );
                )+
            }
        )+
    };
}

class_macro!(
    {
        action:GetterAction,
        GETTER {
            GET_ATTRIBUTE,
            IS_DISPLAYED,
            IS_ENABLED,
            IS_SELECTED,
            // GET_ACCESSBILE_NAME ,
            // GET_ARIA_ROLE,
            // GET_CSS_VALUE,
            // GET_DOM_PROPERTY,
            // GET_LOCATION,
            // GET_SIZE,
            // GET_TAG_NAME,
            // GET_TEXT,
            //
            GET_TITLE ,
            GET_CURRENT_URL
            // GET_PAGE_SOURCE,
            // GET_WINDOW_HANDLE
        }
    },
    {
        action: ElementAction,
        ELEMENT {
            CLEAR,
            CLICK,
            SENDKEYS,
            SUBMIT
        }
    },
    {
        action: WindowAction,
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
        WEB_DRIVER {
            CLOSE,
            NAVIGATE
        }
    },
    {
        action: AlertAction,
        ALERT {
            ACCEPT,
            DISMISS,
            GET_TEXT,
            SEND_KEYS
        }
    },
    {
        action: NavigationAction,
        NAVIGATION {
            BACK,
            FORWARD,
            REFRESH
        }
    },
    {
        action: OptionsAction,
        OPTIONS {
            ADD_COOKIE,
            DELETE_ALL_COOKIES,
            DELETE_COOKIE,
            GET_COOKIE
        }
    },
    {
        action: TargetLocatorAction,
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
        HAS_DOWNLOADS {
            DELETE_DOWNLOADABLE_FILES,
            DOWNLOAD_FILE,
            GET_DOWNLOADABLE_FILES
        }
    },
    {
        action:CustomAction,
        CUSTOM {
            VAR_DECLARATION,
            ASSERT
        }
    },
    {
        action:ArthimaticExpressionAction,
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
        UNARYEXPRESSION  {
            NEGATION,
            GROUPED
        }
    },
    {
        action:BinaryExpressionAction,
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
        CONTROL_FLOW {
            IF,
            ELSE_IF,
            ELSE,
            WHILE,
            FOR
        }
    }
);
