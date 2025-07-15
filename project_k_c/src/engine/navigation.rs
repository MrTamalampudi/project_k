use thirtyfour::WebDriver;

use crate::{ast::teststep::TestStep, class::NavigationEngine};

pub struct Navigation<'a> {
    pub driver: &'a mut WebDriver,
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
