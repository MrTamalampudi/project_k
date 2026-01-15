use std::collections::HashMap;

use logos::Logos;
use macros::EnumToString;

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

            pub fn len(&self) -> usize {
                match self {
                    TokenType::STRING(string) => string.len(),
                    TokenType::IDENTIFIER(ident) => ident.len(),
                    TokenType::NUMBER(num) => num.to_string().len(),
                    TokenType::NONE => 0,
                    TokenType::NEW_LINE => 1,
                    $(TokenType::$keyword =>{
                        let string = if stringify!($keyword) == "EOF"{
                            "EOF".to_string()
                        } else if stringify!($($string)?).len() > 0 {
                            String::from(stringify!($($string)?))
                        } else {
                            stringify!($keyword).replace("_"," ").to_lowercase()
                        };
                        string.len()
                    }
                    ,)*
                }
            }

            pub fn to_string(&self) -> String {
                match self {
                    TokenType::STRING(_) => String::from("string"),
                    TokenType::IDENTIFIER(_) => String::from("identifier"),
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
    CSS,
    VALUE,
    TEXT,
    TAG,
    NAME,
    //prepositions
    FROM,
    TO,
    IN,
    IS,
    //adjective
    CURRENT,
    DISPLAYED,
    ENABLED,
    SELECTED,
    //conjunctions
    AND,
    OR,
    IF,
    ELSE,
    WHILE,
    FOR,
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
    L_SQUARE_BRACE = '[',
    R_SQUARE_BRACE = ']',
    EQUALITY = "==",
    NOT_EQUAL = "!=",
    GREATER_THAN = '>',
    LESSER_THAN = '<',
    GREATER_THAN_EQUAL_TO = ">=",
    LESSER_THAN_EQUAL_TO = "<=",
    //Punctuation
    COMMA = ',',
    //boolean
    TRUE,
    FALSE,
    //High level tokens
    TESTSTEPS,
    CAPABILITIES,
    PREREQUISITE,
    EOF,
    ERROR
);

#[allow(non_camel_case_types)]
#[derive(EnumToString, Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ ]")]
pub enum NTokenType {
    #[token("#testcase")]
    TESTCASE,
    #[token("#testsuite")]
    TESTSUITE,
    #[token("#testplan")]
    TESTPLAN,
    //actions
    #[token("navigate")]
    NAVIGATE,

    #[token("click")]
    CLICK,

    #[token("back")]
    BACK,

    #[token("forward")]
    FORWARD,

    #[token("refresh")]
    REFRESH,

    #[token("get")]
    GET,

    #[token("wait")]
    WAIT,

    #[token("assert")]
    ASSERT,

    #[token("enter")]
    ENTER,

    #[token("close")]
    CLOSE,

    // Nouns
    #[token("attribute")]
    ATTRIBUTE,

    #[token("element")]
    ELEMENT,

    #[token("url")]
    URL,

    #[token("title")]
    TITLE,

    #[token("css")]
    CSS,

    #[token("value")]
    VALUE,

    #[token("text")]
    TEXT,

    #[token("tag")]
    TAG,

    #[token("name")]
    NAME,

    // Prepositions
    #[token("from")]
    FROM,

    #[token("to")]
    TO,

    #[token("in")]
    IN,

    #[token("is")]
    IS,

    // Adjectives
    #[token("current")]
    CURRENT,

    #[token("displayed")]
    DISPLAYED,

    #[token("enabled")]
    ENABLED,

    #[token("selected")]
    SELECTED,

    // Conjunctions
    #[token("and")]
    AND,

    #[token("or")]
    OR,

    #[token("if")]
    IF,

    #[token("else")]
    ELSE,

    #[token("while")]
    WHILE,

    #[token("for")]
    FOR,
    //operators
    #[token("=")]
    ASSIGN_OP,

    #[token("!")]
    NEGATION,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("*")]
    MULTIPLY,

    #[token("/")]
    FORWARDSLASH,

    #[token("%")]
    MODULUS,

    #[token("(")]
    LEFT_PARAN,

    #[token(")")]
    RIGHT_PARAN,

    #[token("{")]
    L_CURLY_BRACE,

    #[token("}")]
    R_CURLY_BRACE,

    #[token("[")]
    L_SQUARE_BRACE,

    #[token("]")]
    R_SQUARE_BRACE,

    #[token("==")]
    EQUALITY,

    #[token("!=")]
    NOT_EQUAL,

    #[token(">")]
    GREATER_THAN,

    #[token("<")]
    LESSER_THAN,

    #[token(">=")]
    GREATER_THAN_EQUAL_TO,

    #[token("<=")]
    LESSER_THAN_EQUAL_TO,

    //Punctuation
    #[token(",")]
    COMMA,

    //boolean
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    BOOL(bool),

    //inputs
    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_owned())]
    STRING(String),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    NUMBER(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    IDENTIFIER(String),

    #[token("\n")]
    NEWLINE,

    EOF,
}
