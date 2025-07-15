use thirtyfour::WebDriver;

use crate::{
    ast::teststep::TestStep,
    class::{Method, NavigationEngine, NAVIGATION},
};

pub struct Navigation<'a> {
    pub driver: &'a mut WebDriver,
}

impl<'a> Navigation<'a> {
    pub async fn new(driver: &mut WebDriver, step: &TestStep) {
        let navigation = Navigation { driver };
        if let Method::NAVIGATION(method) = &step.method {
            match method {
                NAVIGATION::BACK => navigation.BACK(step).await,
                NAVIGATION::REFRESH => navigation.REFRESH(step).await,
                NAVIGATION::FORWARD => navigation.FORWARD(step).await,
            };
        };
    }
}

impl<'a> NavigationEngine for Navigation<'a> {
    async fn REFRESH(&self, step: &TestStep) -> () {
        self.driver.refresh().await;
    }

    async fn BACK(&self, step: &TestStep) -> () {
        self.driver.back().await;
    }

    async fn FORWARD(&self, step: &TestStep) -> () {
        self.driver.forward().await;
    }
}
