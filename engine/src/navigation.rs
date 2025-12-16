use crate::{Engine, EngineResult, e_types};
use ast::teststep::{GetMethod, Teststep};
use class::{Method, NAVIGATION, NavigationEngine};
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn navigation(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::NAVIGATION(method) = &teststep.get_method() {
            match method {
                NAVIGATION::BACK => self.BACK(teststep).await?,
                NAVIGATION::REFRESH => self.REFRESH(teststep).await?,
                NAVIGATION::FORWARD => self.FORWARD(teststep).await?,
            };
        };
        Ok(())
    }
}

impl<'a> NavigationEngine for Engine<'a> {
    e_types!();
    async fn REFRESH(&mut self, _body: &Teststep) -> EngineResult<()> {
        self.driver.refresh().await?;
        Ok(())
    }

    async fn BACK(&mut self, _body: &Teststep) -> EngineResult<()> {
        self.driver.back().await?;
        Ok(())
    }

    async fn FORWARD(&mut self, _body: &Teststep) -> EngineResult<()> {
        self.driver.forward().await?;
        Ok(())
    }
}
