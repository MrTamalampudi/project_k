use crate::enums::{Browser, Capabilities};
use crate::enums::{CapabilityValue, IdentifierValue};
use std::collections::HashMap;

macro_rules! define_tokens {
    ($($keyword:ident $(= $string:literal)?),*) => {
        #[derive(Debug,Clone)]
        #[allow(non_camel_case_types)]
        pub enum TokenType {
            NEW_LINE,
            STRING(String),
            IDENTIFIER(String),
            CAPS(Capabilities),
            //capbilities
            BROWSER(Browser),
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

                    keyword_map.get(token_string).cloned().unwrap_or(TokenType::IDENTIFIER(token_string.to_string()))
            }

            pub fn to_string(&self) -> &str {
                match self {
                    TokenType::STRING(string) | TokenType::IDENTIFIER(string) => string,
                    TokenType::NONE => "none",
                    TokenType::CAPS(caps) => caps.to_string(),
                    TokenType::BROWSER(browser) => browser.to_string(),
                    $(TokenType::$keyword => Box::leak(
                        stringify!($keyword).replace("_"," ").to_lowercase().into_boxed_str()
                    ),)*
                    TokenType::NEW_LINE => "new line"
                }
            }
        }
    };
}

impl TokenType {
    pub fn match_identifier_value(self) -> IdentifierValue {
        match self {
            Self::STRING(string) => IdentifierValue::STRING(string),
            _ => panic!("Not a valid IdentifierValue"),
        }
    }

    pub fn match_capability_value(self) -> CapabilityValue {
        match self {
            Self::BROWSER(browser) => CapabilityValue::BROWSER(browser),
            Self::STRING(string) => CapabilityValue::STRING(string),
            _ => panic!("Not a valid CapabilityValue"),
        }
    }
}

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
    ACTION_RUN,
    //options
    OPTION_TO,
    //Operators
    ASSIGN_OP = '=',
    //High level tokens
    TESTSTEPS,
    CAPABILITIES,
    PREREQUISITE,
    //
    GENERATE,
    UNIQUE_EMAIL,
    EOF
);
