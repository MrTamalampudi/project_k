use std::collections::HashMap;

macro_rules! define_tokens {
    ($($keyword:ident $(= $string:literal)?),*) => {
        #[derive(Debug,Clone,PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum TokenType {
            NEW_LINE,
            STRING(String),
            IDENTIFIER(String),
            NUMBER(isize),
            //capbilities
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
                    TokenType::NUMBER(_) => String::from("number"),
                    TokenType::NONE => "none".to_string(),
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

define_tokens!(
    //FileType
    TESTCASE,
    TESTSUITE,
    TESTPLAN,
    //actions
    NAVIGATE,
    CLICK,
    BACK,
    FORWARD,
    REFRESH,
    GET,
    WAIT,
    ASSERT,
    ENTER,
    CLOSE,
    //nouns
    ATTRIBUTE,
    ELEMENT,
    URL,
    TITLE,
    VAR,
    //prepositions
    FROM,
    TO,
    IN,
    //adjective
    CURRENT,
    //conjunctions
    AND,
    OR,
    IF,
    ELSE,
    //operators
    ASSIGN_OP = '=',
    NEGATION = '!', //from here
    PLUS = '+',
    MINUS = '-',
    MULTIPLY = '*',
    FORWARDSLASH = '/',
    MODULUS = '%',
    LEFT_PARAN = '(',
    RIGHT_PARAN = ')',
    L_CURLY_BRACE = '{',
    R_CURLY_BRACE = '}',
    EQUALITY = "==",
    NOT_EQUAL = "!=",
    GREATER_THAN = '>',
    LESSER_THAN = '<',
    GREATER_THAN_EQUAL_TO = ">=",
    LESSER_THAN_EQUAL_TO = "<=",
    //boolean
    TRUE,
    FALSE,
    //High level tokens
    TESTSTEPS,
    CAPABILITIES,
    PREREQUISITE,
    EOF
);
