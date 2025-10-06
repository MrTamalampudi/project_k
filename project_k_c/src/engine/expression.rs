use crate::ast::{
    expression::{ExpKind, Expr, Literal},
    identifier_value::IdentifierValue,
};

pub trait Evaluation {
    fn eval(expr: &Expr) -> IdentifierValue;
}

pub struct Expression;

impl Evaluation for Expression {
    fn eval(expr: &Expr) -> IdentifierValue {
        match &expr.kind {
            ExpKind::Binary(_, _, _) => BinaryExpr::eval(expr),
            ExpKind::Unary(_, _) => UnaryExpr::eval(expr),
            ExpKind::Lit(_) => LiteralExpr::eval(expr),
        }
    }
}

struct BinaryExpr;

impl Evaluation for BinaryExpr {
    fn eval(expr: &Expr) -> IdentifierValue {
        IdentifierValue::Boolean(None)
    }
}

struct UnaryExpr;

impl Evaluation for UnaryExpr {
    fn eval(expr: &Expr) -> IdentifierValue {
        IdentifierValue::Boolean(None)
    }
}

struct LiteralExpr;

impl Evaluation for LiteralExpr {
    fn eval(expr: &Expr) -> IdentifierValue {
        if let ExpKind::Lit(literal) = expr.kind.clone() {
            return match literal {
                Literal::Number(num) => IdentifierValue::Number(Some(num)),
                Literal::Boolean(bool) => IdentifierValue::Boolean(Some(bool)),
                Literal::String(string) => IdentifierValue::String(Some(string)),
                Literal::Ident(ident, _) => IdentifierValue::Boolean(None),
            };
        } else {
            return IdentifierValue::Boolean(None);
        };
    }
}
