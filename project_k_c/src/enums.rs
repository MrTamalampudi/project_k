use crate::keywords::TokenType;
use std::collections::HashMap;

macro_rules! define_enums {
    (
        $enum_name:ident,
        $($keyword:ident $(= $string:literal)?),*
    ) => {
        #[derive(Debug,Clone,PartialEq)]
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

            pub fn to_vector() -> Vec<String> {
                vec![$(
                    stringify!($keyword).replace("_"," ").to_lowercase()
                ),*]
            }

            pub fn to_string(&self) -> String {
                match self {
                    $enum_name::NONE => "none".to_string(),
                    $($enum_name::$keyword =>
                        stringify!($keyword).replace("_"," ").to_lowercase()
                    ,)*
                }
            }
        }
    };
}

define_enums!(
    //EnumName
    LexingMode,
    //Mode
    TESTSTEPS,
    PREREQUISITE,
    TESTCASE,
    CAPABILITIES
);

impl LexingMode {
    pub fn match_token_type(&self) -> TokenType {
        match self {
            LexingMode::TESTSTEPS => TokenType::TESTSTEPS,
            LexingMode::TESTCASE => TokenType::TESTCASE,
            LexingMode::PREREQUISITE => TokenType::PREREQUISITE,
            LexingMode::CAPABILITIES => TokenType::CAPABILITIES,
            LexingMode::NONE => TokenType::NONE,
        }
    }
}

define_enums!(
    //EnumName
    Capabilities,
    //caps
    BROWSER,
    DRIVERURL
);

impl Capabilities {
    pub fn match_token_type(&self) -> TokenType {
        match self {
            Capabilities::BROWSER => TokenType::CAPS(Capabilities::BROWSER),
            Capabilities::DRIVERURL => TokenType::CAPS(Capabilities::DRIVERURL),
            Capabilities::NONE => TokenType::NONE,
        }
    }

    pub fn is_capability_key_valid(key: &String) -> bool {
        let capability = Capabilities::from_string(key);
        match capability {
            Capabilities::NONE => false,
            _ => true,
        }
    }
}

define_enums!(
    //EnumName
    Browser, CHROME, FIREFOX, EDGE
);

impl Browser {
    pub fn match_token_type(&self) -> TokenType {
        match self {
            Browser::CHROME => TokenType::BROWSER(Browser::CHROME),
            Browser::FIREFOX => TokenType::BROWSER(Browser::FIREFOX),
            Browser::EDGE => TokenType::BROWSER(Browser::EDGE),
            Browser::NONE => TokenType::NONE,
        }
    }

    pub fn match_capability_value(&self) -> CapabilityValue {
        match self {
            Browser::CHROME => CapabilityValue::BROWSER(Browser::CHROME),
            Browser::FIREFOX => CapabilityValue::BROWSER(Browser::FIREFOX),
            Browser::EDGE => CapabilityValue::BROWSER(Browser::EDGE),
            Browser::NONE => CapabilityValue::NONE,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityValue {
    BROWSER(Browser),
    STRING(String),
    NONE,
}
