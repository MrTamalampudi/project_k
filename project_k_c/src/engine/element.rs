use thirtyfour::WebDriver;

use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::testcase_body::GetMethod;
use crate::ast::testcase_body::TestcaseBody;
use crate::class::{ElementEngine, Method, ELEMENT};

pub struct Element<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Element<'a> {
    pub async fn new(driver: &WebDriver, body: &TestcaseBody) {
        let element = Element { driver };
        if let Method::ELEMENT(method) = &body.get_method() {
            match method {
                ELEMENT::CLEAR => {
                    let _ = element.CLEAR(body).await;
                }
                ELEMENT::SENDKEYS => {
                    let _ = element.SENDKEYS(body).await;
                }
                ELEMENT::SUBMIT => {
                    let _ = element.SUBMIT(body).await;
                }
                ELEMENT::CLICK => {
                    let _ = element.CLICK(body).await;
                }
                ELEMENT::GET_ATTRIBUTE => {
                    let _ = element.GET_ATTRIBUTE(body).await;
                }
            };
        };
    }
}

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, _step: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn SUBMIT(&self, _step: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn SENDKEYS(&self, _step: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn GET_ATTRIBUTE(&self, _body: &TestcaseBody) -> Result<Option<String>, String> {
        if let TestcaseBody::GETTER(getter) = _body {
            let locator_arg = getter.arguments.get(LOCATOR_ARGKEY);
            let attribute_arg = getter.arguments.get(ATTRIBUTE_ARGKEY);
            if let (Some(Args::Locator(locator)), Some(Args::String(attribute))) =
                (locator_arg, attribute_arg)
            {
                if let Ok(element) = self.driver.find(locator.to_by()).await {
                    if let Ok(attr) = element.attr(attribute).await {
                        return Ok(attr);
                    }
                }
            }
        }
        Ok(None)
    }

    async fn CLICK(&self, _body: &TestcaseBody) -> Result<(), String> {
        if let TestcaseBody::TESTSTEP(step) = _body {
            if let Args::Locator(locator) = step.arguments.get(LOCATOR_ARGKEY).unwrap() {
                let by = locator.to_by();
                let element = self.driver.find(by).await;
                if let Ok(element) = element {
                    let _ = element.click().await;
                }
            }
        }
        Ok(())
    }
}
