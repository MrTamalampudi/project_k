use crate::{
    ast::{identifier_value::IdentifierValue, teststep::Teststep},
    class::CustomEngine,
    engine::{Engine, EngineResult},
};

impl<'a> CustomEngine for Engine<'a> {
    async fn VAR_DECLARATION(&mut self, _body: &Teststep) -> EngineResult<()> {
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
        Ok(())
    }
}
