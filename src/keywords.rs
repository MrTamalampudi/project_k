use std::collections::HashMap;

macro_rules! define_enums {
    (
        $enum_name:ident,
        $($keyword:ident $(= $string:literal)?),*
    ) => {
        #[derive(Debug,Clone)]
        #[allow(non_camel_case_types)]
        pub enum $enum_name {
            NONE,
            $($keyword),*
        }

        impl $enum_name {
            pub fn from_string(token_string:&str) -> $enum_name {
                let mut keyword_map: HashMap<String,$enum_name> = HashMap::new();
                $(
                    keyword_map.insert(
                        stringify!($keyword).replace("_"," ").to_lowercase(),
                        $enum_name::$keyword
                    );
                )*

                    keyword_map.get(token_string).cloned().unwrap_or($enum_name::NONE)
            }

            pub fn to_string(&self) -> &str {
                match self {
                    $enum_name::NONE => "none",
                    $($enum_name::$keyword => Box::leak(
                        stringify!($keyword).replace("_"," ").to_lowercase().into_boxed_str()
                    ),)*
                }
            }
        }
    };
}

macro_rules! define_tokens {
    ($($keyword:ident $(= $string:literal)?),*) => {
        #[derive(Debug,Clone)]
        #[allow(non_camel_case_types)]
        pub enum TokenType {
            STRING(String),
            IDENTIFIER(String),
            NONE,
            $($keyword),*
        }

        impl TokenType {
            pub fn from_string(token_string:&str) -> TokenType {
                let mut keyword_map: HashMap<String,TokenType> = HashMap::new();
                $(
                    if(stringify!($keyword).contains("ACTION_")){
                        keyword_map.insert(
                            stringify!($keyword).replace("ACTION_","").replace("_"," ").to_lowercase(),
                            TokenType::$keyword
                        );
                    } else if (stringify!($keyword).contains("OPTION_")){
                        keyword_map.insert(
                            stringify!($keyword).replace("OPTION_","").replace("_"," ").to_lowercase(),
                            TokenType::$keyword
                        );
                    } else if stringify!($($string)?).len() > 0{
                        keyword_map.insert(
                            String::from(stringify!($($string)?)),
                            TokenType::$keyword
                        );
                    } else {
                        keyword_map.insert(
                            stringify!($keyword).replace("_"," ").to_lowercase(),
                            TokenType::$keyword
                        );
                    }
                )*

                    keyword_map.get(token_string).cloned().unwrap_or(TokenType::NONE)
            }

            pub fn to_string(&self) -> &str {
                match self {
                    TokenType::STRING(string) | TokenType::IDENTIFIER(string) => string,
                    TokenType::NONE => "none",
                    $(TokenType::$keyword => Box::leak(
                        stringify!($keyword).replace("_"," ").to_lowercase().into_boxed_str()
                    ),)*
                }
            }
        }
    };
}

define_enums!(
    //Enum name
    LexingMode,
    //Mode
    TESTSTEPS,
    PREREQUISITE,
    TESTCASE
);

define_tokens!(
    //FileType
    TESTCASE,
    TESTSUITE,
    TESTPLAN,
    //Actions
    ACTION_NAVIGATE,
    ACTION_CLICK,
    ACTION_UPLOAD,
    ACTION_BACK,
    ACTION_FORWARD,
    //options
    OPTION_TO,
    //Operators
    ASSIGN_OP = '=',
    //High level tokens
    TESTSTEPS,
    CAPABILITIES,
    PREREQUISITE,
    GENERATE,
    UNIQUE_EMAIL,
    EOF
);

pub fn match_lexing_mode_token_type(lexing_mode: &LexingMode) -> TokenType {
    match lexing_mode {
        LexingMode::TESTSTEPS => TokenType::TESTSTEPS,
        LexingMode::TESTCASE => TokenType::TESTCASE,
        LexingMode::PREREQUISITE => TokenType::PREREQUISITE,
        LexingMode::NONE => TokenType::NONE,
    }
}
