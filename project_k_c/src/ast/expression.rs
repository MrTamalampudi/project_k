#![allow(non_camel_case_types, unused)]

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    OperatorExpression(Box<OperatorExpression>),
    GroupedExpression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum LiteralExpression {
    Number(isize),
    Ident(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum OperatorExpression {
    Negation(Expression),
    ArthimaticExpression(ArthimaticExpression),
    ComparisionExpression(ComparisionExpression),
}

//Here LiteralExpression is used for return type
#[derive(Debug, Clone)]
pub enum ArthimaticExpression {
    Plus(Expression, Expression, LiteralExpression),
    Minus(Expression, Expression, LiteralExpression),
    SPMinus(Expression, Expression, LiteralExpression),
    Multiply(Expression, Expression, LiteralExpression),
    Division(Expression, Expression, LiteralExpression),
    Modulus(Expression, Expression, LiteralExpression),
}

#[derive(Debug, Clone)]
pub enum ComparisionExpression {
    Equality(Expression, Expression),
    NotEqual(Expression, Expression),
    GreaterThan(Expression, Expression),
    LesserThan(Expression, Expression),
    GreaterThanEqual(Expression, Expression),
    LesserThanEqual(Expression, Expression),
}
