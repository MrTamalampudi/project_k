use crate::{Engine, EngineResult, e_types};
use ast::teststep::Teststep;
use class::{CONTROL_FLOW, ControlFlowEngine, GetMethod, Method};
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn control_flow(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::CONTROL_FLOW(method) = &teststep.get_method() {
            println!("method {:#?}", method);
            match method {
                CONTROL_FLOW::IF => Box::pin(self.IF(teststep)).await?,
                CONTROL_FLOW::ELSE_IF => self.ELSE_IF(teststep).await?,
                CONTROL_FLOW::ELSE => self.ELSE(teststep).await?,
                CONTROL_FLOW::WHILE => Box::pin(self.WHILE(teststep)).await?,
                CONTROL_FLOW::FOR => Box::pin(self.FOR(teststep)).await?,
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
    async fn FOR(&mut self, _step: &Self::Step) -> Result<(), Self::Error> {
        if let Teststep::For(stmt) = _step.clone() {
            let target = stmt.target;
            let iter_value = match self.eval(&stmt.iter).await {
                Err(e) => return Err(WebDriverError::FatalError(e)),
                Ok(value) => value,
            };
            let iter = self.get_array_expr(&iter_value).await?;
            for i in iter.into_iter() {
                self.testcase.insert_variable_value(target.clone(), i);
                self.execute_body(stmt.body.clone()).await?;
            }
        }
        Ok(())
    }
}
