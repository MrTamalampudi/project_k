use crate::e_types;
use crate::{Engine, EngineResult};
use ast::arguments::{Args, URL_ARGKEY};
use ast::identifier_value::IdentifierValue;
use ast::teststep::GetMethod;
use ast::teststep::Teststep;
use class::{Method, WEB_DRIVER, WebDriverEngine};
use log::info;
use thirtyfour::error::WebDriverError;

impl<'a> Engine<'a> {
    pub async fn webdriver(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::WEB_DRIVER(method) = &teststep.get_method() {
            match method {
                WEB_DRIVER::CLOSE => {
                    let _ = self.CLOSE(teststep).await?;
                }
                WEB_DRIVER::NAVIGATE => {
                    let _ = self.NAVIGATE(teststep).await?;
                }
                WEB_DRIVER::GET_CURRENT_URL => {
                    let _ = self.GET_CURRENT_URL(teststep).await?;
                }
                WEB_DRIVER::GET_PAGE_SOURCE => {
                    let _ = self.GET_PAGE_SOURCE(teststep).await?;
                }
                WEB_DRIVER::GET_TITLE => {
                    let _ = self.GET_TITLE(teststep).await?;
                }
                WEB_DRIVER::GET_WINDOW_HANDLE => {
                    let _ = self.GET_WINDOW_HANDLE(teststep).await?;
                }
            };
        }
        Ok(())
    }
}

impl<'a> WebDriverEngine for Engine<'a> {
    e_types!();
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

    async fn GET_CURRENT_URL(&mut self, _body: &Teststep) -> EngineResult<Option<String>> {
        let url = self.driver.current_url().await?;
        Ok(Some(url.to_string()))
    }
    async fn GET_PAGE_SOURCE(&mut self, _body: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn GET_TITLE(&mut self, _body: &Teststep) -> EngineResult<Option<String>> {
        let title = self.driver.title().await?;
        Ok(Some(title.to_string()))
    }
    async fn GET_WINDOW_HANDLE(&mut self, _body: &Teststep) -> EngineResult<()> {
        Ok(())
    }
}
