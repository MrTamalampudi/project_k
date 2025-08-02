use thirtyfour::WebDriver;

use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::testcase_body::GetMethod;
use crate::ast::testcase_body::TestcaseBody;
use crate::class::{Method, WebDriverEngine, WEB_DRIVER};

pub struct WebDriver_<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> WebDriver_<'a> {
    pub async fn new(driver: &WebDriver, body: &TestcaseBody) {
        let webdriver = WebDriver_ { driver };
        if let Method::WEB_DRIVER(method) = &body.get_method() {
            match method {
                WEB_DRIVER::CLOSE => {
                    let _ = webdriver.CLOSE(body).await;
                }
                WEB_DRIVER::NAVIGATE => {
                    let _ = webdriver.NAVIGATE(body).await;
                }
                WEB_DRIVER::FIND_ELEMENT => {
                    let _ = webdriver.FIND_ELEMENT(body).await;
                }
                WEB_DRIVER::GET_CURRENT_URL => {
                    let _ = webdriver.GET_CURRENT_URL(body).await;
                }
                WEB_DRIVER::GET_PAGE_SOURCE => {
                    let _ = webdriver.GET_PAGE_SOURCE(body).await;
                }
                WEB_DRIVER::GET_TITLE => {
                    let _ = webdriver.GET_TITLE(body).await;
                }
                WEB_DRIVER::GET_WINDOW_HANDLE => {
                    let _ = webdriver.GET_WINDOW_HANDLE(body).await;
                }
            }
        }
    }
}

impl<'a> WebDriverEngine for WebDriver_<'a> {
    async fn NAVIGATE(&self, _body: &TestcaseBody) -> Result<(), String> {
        if let TestcaseBody::TESTSTEP(step) = _body {
            let url = step.arguments.get(URL_ARGKEY).unwrap();
            if let Args::String(url) = url {
                if let Err(_) = self.driver.goto(url).await {
                    panic!("there is an error");
                }
            };
        }
        Ok(())
    }
    async fn CLOSE(&self, _body: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn FIND_ELEMENT(&self, _body: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn GET_CURRENT_URL(&self, _body: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn GET_PAGE_SOURCE(&self, _body: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
    async fn GET_TITLE(&self, _body: &TestcaseBody) -> Result<Option<String>, String> {
        todo!()
    }
    async fn GET_WINDOW_HANDLE(&self, _body: &TestcaseBody) -> Result<(), String> {
        Ok(())
    }
}
