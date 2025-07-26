use thirtyfour::WebDriver;

use crate::{
    ast::{testcase_body::GetMethod, testcase_body::TestcaseBody},
    class::{Method, NavigationEngine, NAVIGATION},
};

pub struct Navigation<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Navigation<'a> {
    pub async fn new(driver: &WebDriver, body: &TestcaseBody) {
        let navigation = Navigation { driver };
        if let Method::NAVIGATION(method) = &body.get_method() {
            match method {
                NAVIGATION::BACK => navigation.BACK(body).await,
                NAVIGATION::REFRESH => navigation.REFRESH(body).await,
                NAVIGATION::FORWARD => navigation.FORWARD(body).await,
            };
        };
    }
}

impl<'a> NavigationEngine for Navigation<'a> {
    async fn REFRESH(&self, _body: &TestcaseBody) -> () {
        self.driver.refresh().await;
    }

    async fn BACK(&self, _body: &TestcaseBody) -> () {
        self.driver.back().await;
    }

    async fn FORWARD(&self, _body: &TestcaseBody) -> () {
        self.driver.forward().await;
    }
}
