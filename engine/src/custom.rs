use thirtyfour::error::WebDriverError;

use crate::{Engine, EngineResult};
use ast::{identifier_value::IdentifierValue, teststep::Teststep};
use class::{CUSTOM, GetMethod, Method};

impl<'a> Engine<'a> {
    pub async fn custom(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::CUSTOM(method) = teststep.get_method() {
            match method {
                CUSTOM::ASSERT => self.ASSERT(teststep).await?,
                CUSTOM::VAR_DECLARATION => self.VAR_DECLARATION(teststep).await?,
            };
        };
        Ok(())
    }
}

impl<'a> Engine<'a> {
    pub async fn VAR_DECLARATION(&mut self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::VarDecl(step) = _body {
            let a = self.eval(&step.rhs).await;
            match a {
                Ok(value) => self
                    .testcase
                    .insert_variable_value(step.name.clone(), value),
                Err(_) => self
                    .testcase
                    .insert_variable_value(step.name.clone(), IdentifierValue::String(None)),
            };
        }
        Ok(())
    }

    async fn ASSERT(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _step {
            let expr_value = self.get_boolean(_step).await?;
            if !expr_value {
                return Err(WebDriverError::FatalError("Assertion Failed".to_string()));
            }
        }
        Ok(())
    }
}
