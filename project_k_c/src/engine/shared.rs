use thirtyfour::{error::WebDriverError, By};

use crate::engine::{
    errors::{INVALID_INPUT, INVALID_LOC_EXPR},
    Engine, EngineResult,
};
use ast::{
    arguments::{Args, ATTRIBUTE_ARGKEY, EXPR_ARGKEY, LOCATOR_ARGKEY},
    identifier_value::IdentifierValue,
    teststep::Teststep,
};

impl<'a> Engine<'a> {
    #[inline]
    pub async fn get_locator(&mut self, _step: &Teststep) -> EngineResult<By> {
        let locator_arg = match _step {
            Teststep::Action(action) => action.arguments.get(LOCATOR_ARGKEY).unwrap(),
            Teststep::Getter(getter) => getter.arguments.get(LOCATOR_ARGKEY).unwrap(),
            _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
        };
        match locator_arg {
            Args::Locator(loc) => Ok(loc.to_by()),
            Args::Expr(expr) => match self.locator_eval(expr).await {
                Ok(loc) => Ok(loc),
                Err(err) => return Err(WebDriverError::FatalError(err)),
            },
            _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
        }
    }

    #[inline]
    pub async fn get_input(&mut self, _step: &Teststep) -> EngineResult<String> {
        let input_arg = match _step {
            Teststep::Action(action) => action.arguments.get(EXPR_ARGKEY).unwrap(),
            Teststep::Getter(getter) => getter.arguments.get(EXPR_ARGKEY).unwrap(),
            _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
        };
        match input_arg {
            Args::Number(num) => Ok(num.to_string()),
            Args::String(str) => Ok(str.to_string()),
            Args::Expr(expr) => match self.input_eval(expr).await {
                Ok(val) => Ok(val),
                Err(err) => return Err(WebDriverError::FatalError(err)),
            },
            _ => return Err(WebDriverError::FatalError(INVALID_INPUT.to_string())),
        }
    }

    //https://www.w3.org/TR/2011/WD-html5-20110525/elements.html#custom-data-attribute
    #[inline]
    pub async fn get_attribute(&mut self, _step: &Teststep) -> EngineResult<String> {
        let attribute_arg = match _step {
            Teststep::Action(action) => action.arguments.get(ATTRIBUTE_ARGKEY).unwrap(),
            Teststep::Getter(getter) => getter.arguments.get(ATTRIBUTE_ARGKEY).unwrap(),
            _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
        };
        match attribute_arg {
            Args::String(str) => Ok(str.to_string()),
            Args::Expr(expr) => match self.attribute_eval(expr).await {
                Ok(val) => Ok(val),
                Err(err) => return Err(WebDriverError::FatalError(err)),
            },
            _ => return Err(WebDriverError::FatalError(INVALID_INPUT.to_string())),
        }
    }

    pub async fn get_boolean(&mut self, _step: &Teststep) -> EngineResult<bool> {
        let input_arg = match _step {
            Teststep::Action(action) => action.arguments.get(EXPR_ARGKEY).unwrap(),
            Teststep::Getter(getter) => getter.arguments.get(EXPR_ARGKEY).unwrap(),
            Teststep::If(stmt) => &Args::Expr(stmt.condition.clone()),
            _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
        };
        if let Args::Expr(expr) = input_arg {
            match self.eval(expr).await {
                Ok(id_val) => match id_val {
                    IdentifierValue::Boolean(bool) => return Ok(bool.unwrap()),
                    _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
                },
                Err(err) => return Err(WebDriverError::FatalError(err)),
            }
        }

        return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string()));
    }
}
