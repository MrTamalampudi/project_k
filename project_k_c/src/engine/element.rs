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
                ELEMENT::CLEAR => element.CLEAR(body).await,
                ELEMENT::SENDKEYS => element.SENDKEYS(body).await,
                ELEMENT::SUBMIT => element.SUBMIT(body).await,
                ELEMENT::CLICK => element.CLICK(body).await,
                ELEMENT::GET_ATTRIBUTE => {}
            };
        };
    }
}

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, _body: &TestcaseBody) -> () {}
    async fn SUBMIT(&self, _body: &TestcaseBody) -> () {}
    async fn SENDKEYS(&self, _body: &TestcaseBody) -> () {}
    async fn GET_ATTRIBUTE(&self, _body: &TestcaseBody) -> String {
        if let TestcaseBody::GETTER(getter) = _body {
            let locator_arg = getter.arguments.get(LOCATOR_ARGKEY);
            let attribute_arg = getter.arguments.get(ATTRIBUTE_ARGKEY);
            if let (Some(Args::Locator(locator)), Some(Args::String(attribute))) =
                (locator_arg, attribute_arg)
            {
                if let Ok(element) = self.driver.find(locator.to_by()).await {
                    if let Ok(attr) = element.attr(attribute).await {
                        match attr {
                            Some(some_) => {
                                return some_;
                            }
                            None => {
                                return String::from("H");
                            }
                        };
                    }
                }
            }
        }
        return String::from("D");
    }

    async fn CLICK(&self, _body: &TestcaseBody) -> () {
        if let TestcaseBody::TESTSTEP(step) = _body {
            if let Args::Locator(locator) = step.arguments.get(LOCATOR_ARGKEY).unwrap() {
                let by = locator.to_by();
                let element = self.driver.find(by).await;
                if let Ok(element) = element {
                    element.click().await;
                }
            }
        }
    }
}
