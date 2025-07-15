use thirtyfour::{By, WebDriver};

use crate::ast::arguments::{Args, LOCATOR_ARGKEY};
use crate::ast::teststep::TestStep;
use crate::class::ElementEngine;
use crate::parser::locator::LocatorStrategy;

pub struct Element<'a> {
    pub driver: &'a mut WebDriver,
}

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, step: &TestStep) -> () {}
    async fn SUBMIT(&self, step: &TestStep) -> () {}
    async fn SENDKEYS(&self, step: &TestStep) -> () {}
    async fn CLICK(&self, step: &TestStep) -> () {
        if let Args::Locator(locator) = step.arguments.get(LOCATOR_ARGKEY).unwrap() {
            let by = locator.to_by();
            let element = self.driver.find(by).await;
            if let Ok(element) = element {
                element.click().await;
            }
        }
    }
}
