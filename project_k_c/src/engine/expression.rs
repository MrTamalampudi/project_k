use crate::{
    ast::{
        expression::{BinOpKind, ExpKind, Expr, Literal, UnOp},
        identifier_value::IdentifierValue,
    },
    engine::{
        errors::{ExpressionEvalResult, EXPECT_LITERAL, INVALID_ADD_OP},
        Engine,
    },
};

pub struct Expression<'a> {
    engine: &'a Engine<'a>,
}

impl<'a> Expression<'a> {
    fn eval(&self, expr: &Expr) -> ExpressionEvalResult {
        match &expr.kind {
            ExpKind::Binary(op, expr1, expr2) => self.binary_eval(op, expr1, expr2),
            ExpKind::Unary(op, expr) => self.unary_eval(op, expr),
            ExpKind::Lit(_) => self.literal_eval(expr),
        }
    }

    fn literal_eval(&self, expr: &Expr) -> ExpressionEvalResult {
        if let ExpKind::Lit(literal) = expr.kind.clone() {
            return match literal {
                Literal::Number(num) => Ok(IdentifierValue::Number(Some(num))),
                Literal::Boolean(bool) => Ok(IdentifierValue::Boolean(Some(bool))),
                Literal::String(string) => Ok(IdentifierValue::String(Some(string))),
                Literal::Ident(ident, _) => {
                    Ok(self.engine.testcase.variables.get(&ident).unwrap().clone())
                }
            };
        } else {
            return Err(EXPECT_LITERAL.to_string());
        };
    }

    fn binary_eval(&self, op: &BinOpKind, expr1: &Expr, expr2: &Expr) -> ExpressionEvalResult {
        use crate::ast::expression::BinOpKind::*;
        let expr1_value = self.eval(expr1)?;
        let expr2_value = self.eval(expr2)?;
        match op {
            Add => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => {
                    Ok(IdentifierValue::Number(Some(num1.unwrap() + num2.unwrap())))
                }
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::String(Some(format!("{}{}", str1.unwrap(), str2.unwrap()))),
                ),
                (IdentifierValue::String(str), IdentifierValue::Number(num)) => Ok(
                    IdentifierValue::String(Some(format!("{}{}", str.unwrap(), num.unwrap()))),
                ),
                (_, _) => Err(INVALID_ADD_OP.to_string()),
            },
            _ => Err(INVALID_ADD_OP.to_string()),
        }
    }

    fn unary_eval(&self, op: &UnOp, expr: &Expr) -> ExpressionEvalResult {
        Err(INVALID_ADD_OP.to_string())
    }
}
