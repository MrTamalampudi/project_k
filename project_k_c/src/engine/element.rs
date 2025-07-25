use thirtyfour::WebDriver;

use crate::ast::arguments::{Args, LOCATOR_ARGKEY};
use crate::ast::teststep::TestStep;
use crate::class::{ElementEngine, Method, ELEMENT};

pub struct Element<'a> {
    pub driver: &'a mut WebDriver,
}

impl<'a> Element<'a> {
    pub async fn new(driver: &mut WebDriver, step: &TestStep) {
        let element = Element { driver };
        if let Method::ELEMENT(method) = &step.method {
            match method {
                ELEMENT::CLEAR => element.CLEAR(step).await,
                ELEMENT::SENDKEYS => element.SENDKEYS(step).await,
                ELEMENT::SUBMIT => element.SUBMIT(step).await,
                ELEMENT::CLICK => element.CLICK(step).await,
                ELEMENT::GET_ATTRIBUTE => element.GET_ATTRIBUTE(step).await,
            };
        };
    }
}

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, _step: &TestStep) -> () {}
    async fn SUBMIT(&self, _step: &TestStep) -> () {}
    async fn SENDKEYS(&self, _step: &TestStep) -> () {}
    async fn GET_ATTRIBUTE(&self, _step: &TestStep) -> () {}
    async fn CLICK(&self, _step: &TestStep) -> () {
        if let Args::Locator(locator) = _step.arguments.get(LOCATOR_ARGKEY).unwrap() {
            let by = locator.to_by();
            let element = self.driver.find(by).await;
            if let Ok(element) = element {
                element.click().await;
            }
        }
    }
}
