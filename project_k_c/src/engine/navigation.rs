use thirtyfour::WebDriver;

use crate::{
    ast::teststep::{GetMethod, Teststep},
    class::{Method, NavigationEngine, NAVIGATION},
    engine::EngineResult,
};

pub struct Navigation<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Navigation<'a> {
    pub async fn new(driver: &WebDriver, body: &Teststep) -> EngineResult<()> {
        let navigation = Navigation { driver };
        if let Method::NAVIGATION(method) = &body.get_method() {
            match method {
                NAVIGATION::BACK => navigation.BACK(body).await?,
                NAVIGATION::REFRESH => navigation.REFRESH(body).await?,
                NAVIGATION::FORWARD => navigation.FORWARD(body).await?,
            };
        };
        Ok(())
    }
}

impl<'a> NavigationEngine for Navigation<'a> {
    async fn REFRESH(&self, _body: &Teststep) -> EngineResult<()> {
        self.driver.refresh().await?;
        Ok(())
    }

    async fn BACK(&self, _body: &Teststep) -> EngineResult<()> {
        self.driver.back().await?;
        Ok(())
    }

    async fn FORWARD(&self, _body: &Teststep) -> EngineResult<()> {
        self.driver.forward().await?;
        Ok(())
    }
}
