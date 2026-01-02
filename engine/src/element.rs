use crate::{Engine, EngineResult};
use ast::{identifier_value::IdentifierValue, teststep::Teststep};
use class::{ELEMENT, GetMethod, Method};
use thirtyfour::error::WebDriverError;

const IS_DISPLAYED_ERROR: &'static str = "Error while evaluating is displayed element action";

impl<'a> Engine<'a> {
    pub async fn element(&mut self, teststep: &Teststep) -> EngineResult<()> {
        if let Method::ELEMENT(method) = &teststep.get_method() {
            match method {
                ELEMENT::CLEAR => self.CLEAR(teststep).await?,
                ELEMENT::SENDKEYS => self.SENDKEYS(teststep).await?,
                ELEMENT::SUBMIT => self.SUBMIT(teststep).await?,
                ELEMENT::CLICK => self.CLICK(teststep).await?,
                ELEMENT::GET_ATTRIBUTE => {
                    self.GET_ATTRIBUTE(teststep).await?;
                }
                ELEMENT::IS_DISPLAYED => {
                    self.IS_DISPLAYED(teststep).await?;
                }
                ELEMENT::GET_ACCESSBILE_NAME => {
                    self.GET_ACCESSBILE_NAME(teststep).await?;
                }
            };
        };
        Ok(())
    }
}

impl<'a> Engine<'a> {
    async fn CLEAR(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SUBMIT(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SENDKEYS(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _step {
            let locator = self.get_locator(_step).await?;
            let input = self.get_input(_step).await?;
            let element = self.driver.find(locator).await?;
            return element.send_keys(input).await;
        }
        Ok(())
    }
    pub async fn GET_ATTRIBUTE(&mut self, _body: &Teststep) -> EngineResult<Option<String>> {
        if let Teststep::Getter(_) = _body {
            let locator = self.get_locator(_body).await?;
            let attribute = self.get_attribute(_body).await?;
            let element = self.driver.find(locator).await?;
            return element.attr(attribute).await;
        }
        Ok(None)
    }

    async fn CLICK(&mut self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(_) = _body {
            let locator = self.get_locator(_body).await?;
            let element = self.driver.find(locator).await?;
            element.click().await?;
        }
        Ok(())
    }

    pub async fn IS_DISPLAYED(&mut self, _step: &Teststep) -> EngineResult<IdentifierValue> {
        let Teststep::Getter(_) = _step else {
            return Err(WebDriverError::FatalError(IS_DISPLAYED_ERROR.to_string()));
        };
        let locator = self.get_locator(_step).await?;
        let element = self.driver.find(locator).await?;
        let displayed = element.is_displayed().await?;
        Ok(IdentifierValue::Boolean(Some(displayed)))
    }

    async fn GET_ACCESSBILE_NAME(&mut self, _step: &Teststep) -> EngineResult<Option<String>> {
        todo!()
    }
}
