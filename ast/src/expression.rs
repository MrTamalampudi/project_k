#![allow(non_camel_case_types, unused)]

use crate::{getter::Getter, identifier_value::IdentifierValue, primitives::Primitives};
use macros::Span;
use span::{Span, SpanData};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(isize),
    Ident(String, Primitives),
    String(String),
    Boolean(bool),
}

impl Literal {
    pub fn to_identifier_value(&self) -> IdentifierValue {
        match self {
            Literal::Boolean(_) => IdentifierValue::Boolean(None),
            Literal::Ident(_, p) => p.to_identifier_value(),
            Literal::String(_) => IdentifierValue::String(None),
            Literal::Number(_) => IdentifierValue::Number(None),
        }
    }
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
    Array(Vec<Expr>),
}

impl ExpKind {
    pub fn to_identifier_value(&self) -> IdentifierValue {
        match self {
            ExpKind::Binary(_, expr, _) => expr.primitive.to_identifier_value(),
            ExpKind::Unary(_, expr) => expr.primitive.to_identifier_value(),
            ExpKind::Lit(lit) => lit.to_identifier_value(),
            ExpKind::Getter(getter) => getter.returns.to_identifier_value(),
            ExpKind::Array(array) => IdentifierValue::Array(None, array.first().unwrap().primitive),
        }
    }
}
