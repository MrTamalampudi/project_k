use std::time::{Duration, SystemTime};

use thirtyfour::{By, WebDriver};

use crate::{
    ast::{
        arguments::{Args, SECS_ARGKEY},
        testcase_body::{GetMethod, TestcaseBody},
    },
    class::{Method, TimeoutsEngine, TIMEOUTS},
    engine::EngineResult,
};

pub struct Timeouts<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Timeouts<'a> {
    pub async fn new(driver: &WebDriver, body: &TestcaseBody) -> EngineResult<()> {
        let timeouts = Timeouts { driver };
        if let Method::TIMEOUTS(method) = body.get_method() {
            match method {
                TIMEOUTS::WAIT => timeouts.WAIT(body).await?,
            }
        }
        Ok(())
    }
}

impl<'a> TimeoutsEngine for Timeouts<'a> {
    async fn WAIT(&self, _step: &TestcaseBody) -> Result<(), thirtyfour::prelude::WebDriverError> {
        if let TestcaseBody::TESTSTEP(step) = _step {
            let secs_arg = step.arguments.get(SECS_ARGKEY);
            if let Some(Args::Number(secs)) = secs_arg {
                let current_time = SystemTime::now();
                let duration = Duration::from_secs(*secs as u64);
                while current_time.elapsed().unwrap() < duration {
                    let _ = self.driver.find(By::XPath("this_is_dummy_tula")).await;
                }
            }
        }
        Ok(())
    }
}
