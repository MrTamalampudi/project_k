use logos::Logos;
use macros::EnumToString;
use manodae::token::TokenKind;

#[allow(non_camel_case_types)]
#[derive(EnumToString, Logos, Debug, Clone, PartialEq)]
#[logos(skip r" ")]
#[logos(skip r"//.*\n")]
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
    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    STRING(String),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    NUMBER(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    IDENTIFIER(String),

    #[token("\n")]
    NEWLINE,

    EOF,
}

impl TokenKind for NTokenType {
    type TokenKind = NTokenType;
    fn error() -> Self::TokenKind {
        NTokenType::EOF
    }
    fn eof() -> Self::TokenKind {
        NTokenType::EOF
    }
}
