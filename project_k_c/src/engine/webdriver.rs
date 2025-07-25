use thirtyfour::WebDriver;

use crate::ast::arguments::{Args, URL_ARGKEY};
use crate::ast::teststep::TestStep;
use crate::class::{Method, WebDriverEngine, WEB_DRIVER};

pub struct WebDriver_<'a> {
    pub driver: &'a mut WebDriver,
}

impl<'a> WebDriver_<'a> {
    pub async fn new(driver: &mut WebDriver, step: &TestStep) {
        let webdriver = WebDriver_ { driver };
        if let Method::WEB_DRIVER(method) = &step.method {
            match method {
                WEB_DRIVER::CLOSE => {
                    webdriver.CLOSE(step).await;
                }
                WEB_DRIVER::NAVIGATE => {
                    webdriver.NAVIGATE(step).await;
                }
                WEB_DRIVER::FIND_ELEMENT => {
                    webdriver.FIND_ELEMENT(step).await;
                }
                WEB_DRIVER::GET_CURRENT_URL => {
                    webdriver.GET_CURRENT_URL(step).await;
                }
                WEB_DRIVER::GET_PAGE_SOURCE => {
                    webdriver.GET_PAGE_SOURCE(step).await;
                }
                WEB_DRIVER::GET_TITLE => {
                    webdriver.GET_TITLE(step).await;
                }
                WEB_DRIVER::GET_WINDOW_HANDLE => {
                    webdriver.GET_WINDOW_HANDLE(step).await;
                }
            }
        }
    }
}

impl<'a> WebDriverEngine for WebDriver_<'a> {
    async fn NAVIGATE(&self, _step: &TestStep) -> () {
        let url = _step.arguments.get(URL_ARGKEY).unwrap();
        if let Args::String(url) = url {
            if let Err(_) = self.driver.goto(url).await {
                panic!("there is an error");
            }
        };
    }
    async fn CLOSE(&self, _step: &TestStep) -> () {}
    async fn FIND_ELEMENT(&self, _step: &TestStep) -> () {}
    async fn GET_CURRENT_URL(&self, _step: &TestStep) -> () {}
    async fn GET_PAGE_SOURCE(&self, _step: &TestStep) -> () {}
    async fn GET_TITLE(&self, _step: &TestStep) -> String {
        todo!()
    }
    async fn GET_WINDOW_HANDLE(&self, _step: &TestStep) -> () {}
}
