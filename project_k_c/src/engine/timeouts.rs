use std::time::{Duration, SystemTime};

use thirtyfour::{error::WebDriverError, By};

use crate::{
    ast::{
        arguments::{Args, EXPR_ARGKEY},
        identifier_value::IdentifierValue,
        teststep::{GetMethod, Teststep},
    },
    e_types,
    engine::{Engine, EngineResult},
};
use class::{Method, TimeoutsEngine, TIMEOUTS};

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
    e_types!();
    async fn WAIT(&mut self, _step: &Teststep) -> Result<(), thirtyfour::prelude::WebDriverError> {
        if let Teststep::Action(step) = _step {
            let secs_arg = step.arguments.get(EXPR_ARGKEY);
            if let Some(Args::Expr(secs_expr)) = secs_arg {
                let sec = self.eval(secs_expr).await;
                match sec {
                    Ok(IdentifierValue::Number(value)) => {
                        let current_time = SystemTime::now();
                        let duration = Duration::from_secs(value.unwrap() as u64);
                        while current_time.elapsed().unwrap() < duration {
                            let _ = self.driver.find(By::XPath("this_is_dummy_tula")).await;
                        }
                    }
                    Ok(_) | Err(_) => {
                        return Err(WebDriverError::FatalError(
                            "Error while parsing time".to_string(),
                        ))
                    }
                }
            }
        }
        Ok(())
    }
}
