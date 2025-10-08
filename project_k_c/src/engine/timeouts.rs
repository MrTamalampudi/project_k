use std::time::{Duration, SystemTime};

use thirtyfour::By;

use crate::{
    ast::{
        arguments::{Args, SECS_ARGKEY},
        teststep::{GetMethod, Teststep},
    },
    class::{Method, TimeoutsEngine, TIMEOUTS},
    engine::{Engine, EngineResult},
};

impl<'a> Engine<'a> {
    pub async fn timeouts(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::TIMEOUTS(method) = teststep.get_method() {
            match method {
                TIMEOUTS::WAIT => self.WAIT(teststep).await?,
            }
        }
        Ok(())
    }
}

impl<'a> TimeoutsEngine for Engine<'a> {
    async fn WAIT(&mut self, _step: &Teststep) -> Result<(), thirtyfour::prelude::WebDriverError> {
        if let Teststep::Action(step) = _step {
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
