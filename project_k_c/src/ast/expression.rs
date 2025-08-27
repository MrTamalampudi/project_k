#![allow(non_camel_case_types, unused)]

use crate::ast::primitives::Primitives;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpression),
    OperatorExpression(Box<OperatorExpression>),
    GroupedExpression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpression {
    Number(isize),
    Ident(String, Primitives),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorExpression {
    Negation(Expression),
    ArthimaticExpression(ArthimaticExpression),
    ComparisionExpression(ComparisionExpression),
}

//Here LiteralExpression is used for return type
#[derive(Debug, Clone, PartialEq)]
pub enum ArthimaticExpression {
    Plus(Expression, Expression, LiteralExpression),
    Minus(Expression, Expression, LiteralExpression),
    SPMinus(Expression, Expression, LiteralExpression),
    Multiply(Expression, Expression, LiteralExpression),
    Division(Expression, Expression, LiteralExpression),
    Modulus(Expression, Expression, LiteralExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisionExpression {
    Equality(Expression, Expression),
    NotEqual(Expression, Expression),
    GreaterThan(Expression, Expression),
    LesserThan(Expression, Expression),
    GreaterThanEqual(Expression, Expression),
    LesserThanEqual(Expression, Expression),
}
