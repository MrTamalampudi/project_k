use thirtyfour::WebDriver;

use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::teststep::GetMethod;
use crate::ast::teststep::Teststep;
use crate::class::{ElementEngine, Method, ELEMENT};
use crate::engine::EngineResult;

pub struct Element<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Element<'a> {
    pub async fn new(driver: &WebDriver, body: &Teststep) -> EngineResult<()> {
        let element = Element { driver };
        if let Method::ELEMENT(method) = &body.get_method() {
            match method {
                ELEMENT::CLEAR => {
                    let _ = element.CLEAR(body).await?;
                }
                ELEMENT::SENDKEYS => {
                    let _ = element.SENDKEYS(body).await?;
                }
                ELEMENT::SUBMIT => {
                    let _ = element.SUBMIT(body).await?;
                }
                ELEMENT::CLICK => {
                    let _ = element.CLICK(body).await?;
                }
                ELEMENT::GET_ATTRIBUTE => {
                    let _ = element.GET_ATTRIBUTE(body).await?;
                }
            };
        };
        Ok(())
    }
}

impl<'a> ElementEngine for Element<'a> {
    async fn CLEAR(&self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SUBMIT(&self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SENDKEYS(&self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn GET_ATTRIBUTE(&self, _body: &Teststep) -> EngineResult<Option<String>> {
        if let Teststep::GETTER(getter) = _body {
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

    async fn CLICK(&self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(step) = _body {
            if let Args::Locator(locator) = step.arguments.get(LOCATOR_ARGKEY).unwrap() {
                let by = locator.to_by();
                let element = self.driver.find(by).await?;
                element.click().await?;
            }
        }
        Ok(())
    }
}
