use slr_parser::terminal::Terminal;

use crate::enums::CapabilityValue;
use crate::enums::{Browser, Capabilities};
use std::collections::HashMap;

macro_rules! define_tokens {
    ($($keyword:ident $(= $string:literal)?),*) => {
        #[derive(Debug,Clone,PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum TokenType {
            NEW_LINE,
            STRING(String),
            IDENTIFIER(String),
            CAPS(Capabilities),
            //capbilities
            BROWSER(Browser),
            XPATH(String),
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

            pub fn to_string(&self) -> String {
                match self {
                    TokenType::STRING(_) => String::from("string"),
                    TokenType::IDENTIFIER(_) => String::from("identifier"),
                    TokenType::XPATH(_)  => String::from("xpath"),
                    TokenType::NONE => "none".to_string(),
                    TokenType::CAPS(caps) => caps.to_string().clone(),
                    TokenType::BROWSER(browser) => browser.to_string().clone(),
                    $(TokenType::$keyword =>{
                        if stringify!($keyword) == "EOF"{
                            "EOF".to_string()
                        } else if stringify!($($string)?).len() > 0 {
                            String::from(stringify!($($string)?))
                        } else {
                            stringify!($keyword).replace("_"," ").to_lowercase()
                        }}
                    ,)*
                    TokenType::NEW_LINE => "new line".to_string(),
                }
            }
        }
    };
}

impl Terminal for TokenType {
    fn get_ending_token() -> String {
        "EOF".to_string()
    }
    fn to_string_c(&self) -> String {
        self.to_string()
    }
    fn get_value(&self) -> Option<String> {
        match self {
            TokenType::STRING(string) => Some(string.clone()),
            TokenType::IDENTIFIER(identifier) => Some(identifier.clone()),
            _ => None,
        }
    }
}

impl TokenType {
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
    // ***** actions
    NAVIGATE,
    CLICK,
    BACK,
    FORWARD,
    REFRESH,
    GET,
    // ***** prepositions
    FROM,
    //nouns
    ATTRIBUTE,
    ELEMENT,
    TO,
    // ***** operators
    ASSIGN_OP = '=',
    //High level tokens
    TESTSTEPS,
    CAPABILITIES,
    PREREQUISITE,
    EOF
);
