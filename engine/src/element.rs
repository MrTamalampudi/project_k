use crate::{Engine, EngineResult};
use ast::Teststep;
use class::{ELEMENT, GetMethod, Method};

impl<'a> Engine<'a> {
    pub async fn element(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::ELEMENT(method) = &teststep.get_method() {
            match method {
                ELEMENT::CLEAR => self.CLEAR(teststep).await?,
                ELEMENT::SENDKEYS => self.SENDKEYS(teststep).await?,
                ELEMENT::SUBMIT => self.SUBMIT(teststep).await?,
                ELEMENT::CLICK => self.CLICK(teststep).await?,
            };
        };
        Ok(())
    }
}

impl<'a> Engine<'a> {
    async fn CLEAR(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SUBMIT(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SENDKEYS(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _step {
            let locator = self.get_locator(_step).await?;
            let input = self.get_input(_step).await?;
            let element = self.driver.find(locator).await?;
            return element.send_keys(input).await;
        }
        Ok(())
    }

    async fn CLICK(&mut self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _body {
            let locator = self.get_locator(_body).await?;
            let element = self.driver.find(locator).await?;
            element.click().await?;
        }
        Ok(())
    }
}
