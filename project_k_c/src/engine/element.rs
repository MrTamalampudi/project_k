use thirtyfour::{By, WebDriver};

use crate::ast::arguments::Args;
use crate::ast::teststep::TestStep;
use crate::class::ElementEngine;
use crate::parser::locator::LocatorStrategy;

pub struct Element<'a>(pub &'a mut WebDriver);

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, step: &TestStep) -> () {}
    async fn SUBMIT(&self, step: &TestStep) -> () {}
    async fn SENDKEYS(&self, step: &TestStep) -> () {}
    async fn CLICK(&self, step: &TestStep) -> () {
        if let Args::Locator(locator) = step.arguments.get(0).unwrap() {
            let by = locator.to_by();
            let element = self.0.find(by).await;
            if let Ok(element) = element {
                element.click().await;
            }
        }
    }
}
