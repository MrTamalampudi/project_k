#![allow(non_camel_case_types, unused)]

use crate::{getter::Getter, primitives::Primitives};
use macros::Span;
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(isize),
    Ident(String, Primitives),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Span)]
pub struct Expr {
    pub kind: ExpKind,
    pub span: Span,
    pub primitive: Primitives,
}

impl Expr {
    pub fn boolean(&self) -> bool {
        self.primitive == Primitives::Boolean
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinOpKind {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `%` operator (modulus)
    Rem,
    /// The `&&` operator (logical and)
    And,
    /// The `||` operator (logical or)
    Or,
    /// The `==` operator (equality)
    Eq,
    /// The `<` operator (less than)
    Lt,
    /// The `<=` operator (less than or equal to)
    Le,
    /// The `!=` operator (not equal to)
    Ne,
    /// The `>=` operator (greater than or equal to)
    Ge,
    /// The `>` operator (greater than)
    Gt,
}

impl BinOpKind {
    pub fn is_comparision(&self) -> bool {
        use BinOpKind::*;
        match self {
            Eq | Ne | Lt | Le | Gt | Ge => true,
            Add | Sub | Mul | Div | Rem | And | Or => false,
        }
    }

    pub fn is_bool_op(&self) -> bool {
        use BinOpKind::*;
        match self {
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => true,
            Add | Sub | Mul | Div | Rem => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpKind {
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Lit(Literal),
    Getter(Getter),
}
