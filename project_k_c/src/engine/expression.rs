use std::ops::BitXor;

use crate::{
    ast::{
        expression::{BinOpKind, ExpKind, Expr, Literal, UnOp},
        identifier_value::IdentifierValue,
    },
    engine::{
        errors::{
            ExpressionEvalResult, EXPECT_LITERAL, INT_OVERFLOW, INVALID_ADD_OP, INVALID_AND_OP,
            INVALID_EQ_OP, INVALID_OR_OP, INVALID_SUB_OP, INVALID_UNARY_OP,
        },
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
            Sub => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Number(Some(match num1.unwrap().checked_sub(num2.unwrap()) {
                        Some(res) => res,
                        None => return Err(INT_OVERFLOW.to_string()),
                    })),
                ),
                (_, _) => Err(INVALID_SUB_OP.to_string()),
            },
            Mul => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Number(Some(match num1.unwrap().checked_mul(num2.unwrap()) {
                        Some(res) => res,
                        None => return Err(INT_OVERFLOW.to_string()),
                    })),
                ),
                (_, _) => Err(INVALID_SUB_OP.to_string()),
            },
            Div => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Number(Some(match num1.unwrap().checked_div(num2.unwrap()) {
                        Some(res) => res,
                        None => return Err(INT_OVERFLOW.to_string()),
                    })),
                ),
                (_, _) => Err(INVALID_SUB_OP.to_string()),
            },
            Rem => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Number(Some(match num1.unwrap().checked_rem(num2.unwrap()) {
                        Some(res) => res,
                        None => return Err(INT_OVERFLOW.to_string()),
                    })),
                ),
                (_, _) => Err(INVALID_SUB_OP.to_string()),
            },
            And => match (expr1_value, expr2_value) {
                (IdentifierValue::Boolean(bool1), IdentifierValue::Boolean(bool2)) => Ok(
                    IdentifierValue::Boolean(Some(bool1.unwrap() && bool2.unwrap())),
                ),
                (_, _) => Err(INVALID_AND_OP.to_string()),
            },
            Or => match (expr1_value, expr2_value) {
                (IdentifierValue::Boolean(bool1), IdentifierValue::Boolean(bool2)) => Ok(
                    IdentifierValue::Boolean(Some(bool1.unwrap() || bool2.unwrap())),
                ),
                (_, _) => Err(INVALID_OR_OP.to_string()),
            },
            Eq => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().eq(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().eq(str2.unwrap().trim()))),
                ),
                (IdentifierValue::Boolean(bool1), IdentifierValue::Boolean(bool2)) => Ok(
                    IdentifierValue::Boolean(Some(!bool1.unwrap().bitxor(bool2.unwrap()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
            Ne => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().ne(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().ne(str2.unwrap().trim()))),
                ),
                (IdentifierValue::Boolean(bool1), IdentifierValue::Boolean(bool2)) => Ok(
                    IdentifierValue::Boolean(Some(bool1.unwrap().bitxor(bool2.unwrap()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
            Lt => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().lt(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().lt(str2.unwrap().trim()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
            Le => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().le(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().le(str2.unwrap().trim()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
            Gt => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().gt(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().gt(str2.unwrap().trim()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
            Ge => match (expr1_value, expr2_value) {
                (IdentifierValue::Number(num1), IdentifierValue::Number(num2)) => Ok(
                    IdentifierValue::Boolean(Some(num1.unwrap().ge(&num2.unwrap()))),
                ),
                (IdentifierValue::String(str1), IdentifierValue::String(str2)) => Ok(
                    IdentifierValue::Boolean(Some(str1.unwrap().trim().ge(str2.unwrap().trim()))),
                ),
                (_, _) => Err(INVALID_EQ_OP.to_string()),
            },
        }
    }

    fn unary_eval(&self, op: &UnOp, expr: &Expr) -> ExpressionEvalResult {
        if &UnOp::Not == op {
            let value = self.eval(expr)?;
            if let IdentifierValue::Boolean(bool) = value {
                return Ok(IdentifierValue::Boolean(Some(!bool.unwrap())));
            } else {
                return Err(INVALID_UNARY_OP.to_string());
            }
        } else {
            return Err(INVALID_UNARY_OP.to_string());
        }
    }
}
