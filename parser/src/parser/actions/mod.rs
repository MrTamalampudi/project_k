pub(super) mod binary_expr;
pub(super) mod control_flow;
pub(super) mod custom;
pub(super) mod driver;
pub(super) mod element;
pub(super) mod getter;
pub(super) mod literal_expression;
pub(super) mod navigation;
pub(super) mod shared;
pub(super) mod timeouts;
pub(super) mod unary_expr;

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
macro_rules! a_types {
    () => {
        type AST = TestCase;
        type Token = Token;
        type TranslatorStack = TranslatorStack;
        type Error<Token> = ParseError<Token>;
    };
}
