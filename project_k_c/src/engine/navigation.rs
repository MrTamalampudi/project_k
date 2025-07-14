use thirtyfour::WebDriver;

use crate::{ast::teststep::TestStep, class::NavigationEngine};

pub struct Navigation(WebDriver);

impl NavigationEngine for Navigation {
    async fn REFRESH(&self, step: &TestStep) -> () {
        self.0.refresh().await;
    }

    async fn BACK(&self, step: &TestStep) -> () {
        self.0.back().await;
    }

    async fn FORWARD(&self, step: &TestStep) -> () {
        self.0.forward().await;
    }
}
