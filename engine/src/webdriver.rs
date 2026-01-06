use crate::{Engine, EngineResult};
use ast::ArgKeys::{Args, URL_ARGKEY};
use ast::IdentifierValue;
use ast::Teststep;
use class::{GetMethod, Method, WEB_DRIVER};
use log::info;
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn webdriver(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::WEB_DRIVER(method) = &teststep.get_method() {
            match method {
                WEB_DRIVER::CLOSE => self.CLOSE(teststep).await?,
                WEB_DRIVER::NAVIGATE => self.NAVIGATE(teststep).await?,
            };
        }
        Ok(())
    }
}

impl<'a> Engine<'a> {
    async fn NAVIGATE(&mut self, _body: &Teststep) -> EngineResult<()> {
        info!("Navigated to ");
        if let Teststep::Action(step) = _body {
            let url = step.arguments.get(URL_ARGKEY).unwrap();
            if let Args::Expr(url) = url {
                let url = self.eval(url).await;
                match url {
                    Ok(url_) => {
                        if let IdentifierValue::String(uu) = url_ {
                            let _ = self.driver.goto(uu.unwrap()).await;
                        }
                    }
                    Err(_) => return Err(WebDriverError::FatalError("navigate error".to_string())),
                };
                info!("Navigated to ");
            };
        }
        Ok(())
    }
    async fn CLOSE(&mut self, _body: &Teststep) -> EngineResult<()> {
        self.driver.close_window().await?;
        info!("closed browser");
        Ok(())
    }
}
