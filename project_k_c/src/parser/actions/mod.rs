pub mod binary_expr;
pub mod conditional;
pub mod custom;
pub mod driver;
pub mod element;
pub mod expression;
pub mod literal_expression;
pub mod navigation;
pub mod shared;
pub mod timeouts;
pub mod unary_expr;

#[macro_export]
macro_rules! pop_expr {
    ($pop_expr:expr,$error:ident,$token:ident) => {{
        match $pop_expr {
            Ok(expr) => expr,
            Err((error, span)) => {
                $error.push_error(&$token, &span, error);
                return;
            }
        }
    }};
}

#[macro_export]
macro_rules! types {
    () => {
        type AST = TestCase;
        type Token = Token;
        type TranslatorStack = TranslatorStack;
        type Error<Token> = ParseError<Token>;
    };
}
