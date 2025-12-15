use crate::{
    ast::teststep::{GetMethod, Teststep},
    e_types,
    engine::{Engine, EngineResult},
};
use class::{ConditionalStmtEngine, Method, CONDITIONAL_STMT};
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn conditional(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::CONDITIONAL_STMT(method) = &teststep.get_method() {
            match method {
                CONDITIONAL_STMT::IF => Box::pin(self.IF(teststep)).await?,
                CONDITIONAL_STMT::ELSE_IF => self.ELSE_IF(teststep).await?,
                CONDITIONAL_STMT::ELSE => self.ELSE(teststep).await?,
            };
        };
        Ok(())
    }
}

impl<'a> ConditionalStmtEngine for Engine<'a> {
    e_types!();
    async fn IF(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::If(stmt) = _step.clone() {
            let condition = self.get_boolean(_step).await?;
            if condition {
                self.execute_body(stmt.body).await?;
            } else {
                let body = *stmt.or_else;
                if let Some(body) = body {
                    Box::pin(self.IF(&Teststep::If(body))).await?;
                }
            }
        }
        Ok(())
    }
    async fn ELSE_IF(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn ELSE(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
}
