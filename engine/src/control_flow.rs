use crate::{Engine, EngineResult, e_types};
use ast::teststep::{GetMethod, Teststep};
use class::{CONTROL_FLOW, ControlFlowEngine, Method};
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn control_flow(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::CONTROL_FLOW(method) = &teststep.get_method() {
            match method {
                CONTROL_FLOW::IF => Box::pin(self.IF(teststep)).await?,
                CONTROL_FLOW::ELSE_IF => self.ELSE_IF(teststep).await?,
                CONTROL_FLOW::ELSE => self.ELSE(teststep).await?,
                CONTROL_FLOW::WHILE => Box::pin(self.WHILE(teststep)).await?,
            };
        };
        Ok(())
    }
}

impl<'a> ControlFlowEngine for Engine<'a> {
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
    async fn WHILE(&mut self, _step: &Self::Step) -> EngineResult<()> {
        if let Teststep::If(stmt) = _step.clone() {
            let mut condition = self.get_boolean(_step).await?;
            while condition {
                self.execute_body(stmt.body.clone()).await?;
                condition = self.get_boolean(_step).await?;
            }
        }
        Ok(())
    }
}
