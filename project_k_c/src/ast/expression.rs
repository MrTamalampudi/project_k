#![allow(non_camel_case_types, unused)]

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: BinOpKind,
}

#[derive(Debug, Clone)]
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
