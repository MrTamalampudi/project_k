mod binary_expr;
mod control_flow;
mod custom;
mod driver;
mod element;
mod getter;
mod literal_expression;
mod navigation;
mod shared;
mod timeouts;
mod unary_expr;

pub(super) use binary_expr::BinaryExpression;
pub(super) use control_flow::ControlFlow;
pub(super) use custom::Custom;
pub(super) use driver::Driver;
pub(super) use element::Element;
pub(super) use getter::Getter;
pub(super) use literal_expression::LiteralExpression;
pub(super) use navigation::Navigation;
pub(super) use shared::Shared;
pub(super) use timeouts::Timeouts;
pub(super) use unary_expr::UnaryExpression;

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
