use crate::keywords::TokenType;
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

define_enums!(
    //Enum name
    LexingMode,
    //Mode
    TESTSTEPS,
    PREREQUISITE,
    TESTCASE
);

#[derive(Debug, Clone)]
pub enum IdentifierValue {
    STRING(String),
}

impl LexingMode {
    pub fn match_token_type(&self) -> TokenType {
        match self {
            LexingMode::TESTSTEPS => TokenType::TESTSTEPS,
            LexingMode::TESTCASE => TokenType::TESTCASE,
            LexingMode::PREREQUISITE => TokenType::PREREQUISITE,
            LexingMode::NONE => TokenType::NONE,
        }
    }
}
