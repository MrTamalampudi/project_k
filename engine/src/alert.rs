use ast::Teststep;
use class::{ALERT, GetMethod, Method};

use crate::{Engine, EngineResult};

impl<'a> Engine<'a> {
    pub async fn alert(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::ALERT(method) = teststep.get_method() {
            match method {
                ALERT::ACCEPT => self.ACCEPT(teststep).await?,
                ALERT::DISMISS => self.DISMISS(teststep).await?,
                ALERT::SEND_KEYS => self.ALERT_SENDKEYS(teststep).await?,
            };
        };
        Ok(())
    }

    async fn ACCEPT(&mut self, _step: &Teststep) -> EngineResult<()> {
        self.driver.accept_alert().await?;
        Ok(())
    }
    async fn DISMISS(&mut self, _step: &Teststep) -> EngineResult<()> {
        self.driver.dismiss_alert().await?;
        Ok(())
    }
    async fn ALERT_SENDKEYS(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _step {
            let input = self.get_input(_step).await?;
            self.driver.send_alert_text(input).await?;
        }
        Ok(())
    }
}
