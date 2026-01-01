use crate::{expression::Expr, primitives::Primitives};
use class::{CUSTOM, Method};
use macros::{Method, Span};
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq, Span, Method)]
pub struct VarDecl {
    pub name: String,
    pub type_: Primitives,
    pub rhs: Expr,
    pub method: Method,
    pub span: Span,
}

impl VarDecl {
    pub fn new(name: String, type_: Primitives, rhs: Expr, span: Span) -> VarDecl {
        VarDecl {
            name,
            type_,
            rhs,
            method: Method::CUSTOM(CUSTOM::VAR_DECLARATION),
            span,
        }
    }
}
