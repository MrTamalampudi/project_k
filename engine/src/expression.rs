use std::ops::BitXor;

use thirtyfour::By;

use crate::{
    Engine,
    errors::{
        ARRAY_EVAL, EXPECT_LITERAL, ExpressionEvalResult, INT_OVERFLOW, INVALID_ADD_OP,
        INVALID_AND_OP, INVALID_EQ_OP, INVALID_INPUT, INVALID_LOC_EXPR, INVALID_OR_OP,
        INVALID_SUB_OP, INVALID_UNARY_OP,
    },
};
use ast::{
    expression::{BinOpKind, ExpKind, Expr, Literal, UnOp},
    getter::Getter,
    identifier_value::IdentifierValue,
    locator::LocatorStrategy,
    teststep::Teststep,
};
use class::{ELEMENT, ElementEngine, Method, WEB_DRIVER, WebDriverEngine};

impl<'a> Engine<'a> {
    pub async fn eval(&mut self, expr: &Expr) -> ExpressionEvalResult {
        match &expr.kind {
            ExpKind::Binary(op, expr1, expr2) => self.binary_eval(op, expr1, expr2).await,
            ExpKind::Unary(op, expr) => self.unary_eval(op, expr).await,
            ExpKind::Lit(_) => self.literal_eval(expr),
            ExpKind::Array(expr_arr) => self.array_eval(expr_arr).await,
            ExpKind::Getter(getter) => self.getter_eval(getter).await,
        }
    }

    //
    async fn array_eval(&mut self, expr_arr: &Vec<Expr>) -> ExpressionEvalResult {
        let mut array = vec![];
        for expr in expr_arr.iter() {
            let value = Box::pin(self.eval(expr)).await?;
            array.push(value);
        }
        let primitive = if let Some(value) = array.first() {
            value.to_primitive()
        } else {
            return Err(ARRAY_EVAL.to_string());
        };
        return Ok(IdentifierValue::Array(Some(array), primitive));
    }

    fn literal_eval(&self, expr: &Expr) -> ExpressionEvalResult {
        if let ExpKind::Lit(literal) = expr.kind.clone() {
            return match literal {
                Literal::Number(num) => Ok(IdentifierValue::Number(Some(num))),
                Literal::Boolean(bool) => Ok(IdentifierValue::Boolean(Some(bool))),
                Literal::String(string) => Ok(IdentifierValue::String(Some(string))),
                Literal::Ident(ident, _) => {
                    Ok(self.testcase.variables.get(&ident).unwrap().clone())
                }
            };
        } else {
            return Err(EXPECT_LITERAL.to_string());
        };
    }

    async fn binary_eval(
        &mut self,
        op: &BinOpKind,
        expr1: &Expr,
        expr2: &Expr,
    ) -> ExpressionEvalResult {
        use ast::expression::BinOpKind::*;
        let expr1_value = Box::pin(self.eval(expr1)).await?;
        let expr2_value = Box::pin(self.eval(expr2)).await?;
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

    #[inline]
    async fn unary_eval(&mut self, op: &UnOp, expr: &Expr) -> ExpressionEvalResult {
        if &UnOp::Not == op {
            let value = Box::pin(self.eval(expr)).await?;
            if let IdentifierValue::Boolean(bool) = value {
                return Ok(IdentifierValue::Boolean(Some(!bool.unwrap())));
            } else {
                return Err(INVALID_UNARY_OP.to_string());
            }
        } else {
            return Err(INVALID_UNARY_OP.to_string());
        }
    }

    #[inline]
    async fn getter_eval(&mut self, getter: &Getter) -> ExpressionEvalResult {
        let a = match getter.method {
            Method::ELEMENT(ELEMENT::GET_ATTRIBUTE) => {
                self.GET_ATTRIBUTE(&Teststep::Getter(getter.clone())).await
            }
            Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL) => {
                self.GET_CURRENT_URL(&Teststep::Getter(getter.clone()))
                    .await
            }
            Method::WEB_DRIVER(WEB_DRIVER::GET_TITLE) => {
                self.GET_TITLE(&Teststep::Getter(getter.clone())).await
            }
            _ => return Err("".to_string()),
        };

        match a {
            Ok(ok) => Ok(IdentifierValue::String(ok)),
            Err(_) => Err("".to_string()),
        }
    }

    #[inline]
    pub async fn locator_eval(&mut self, expr: &Expr) -> Result<By, String> {
        let eval_value = Box::pin(self.eval(expr)).await?;
        if let IdentifierValue::String(locator_string) = eval_value {
            if let Some(locator_string) = locator_string {
                return Ok(LocatorStrategy::parse(&locator_string).to_by());
            }
        }
        return Err(INVALID_LOC_EXPR.to_string());
    }

    #[inline]
    pub async fn input_eval(&mut self, expr: &Expr) -> Result<String, String> {
        let eval_value = self.eval(expr).await?;
        return match eval_value {
            IdentifierValue::Boolean(bool) => Ok(bool.unwrap().to_string()),
            IdentifierValue::Number(num) => Ok(num.unwrap().to_string()),
            IdentifierValue::String(str) => Ok(str.unwrap()),
            _ => Err(INVALID_INPUT.to_string()),
        };
    }

    #[inline]
    pub async fn attribute_eval(&mut self, expr: &Expr) -> Result<String, String> {
        let eval_value = Box::pin(self.eval(expr)).await?;
        return match eval_value {
            IdentifierValue::String(str) => Ok(str.unwrap()),
            _ => Err(INVALID_INPUT.to_string()),
        };
    }
}
