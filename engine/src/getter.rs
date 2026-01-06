use ast::{IdentifierValue, Teststep};
use class::{GETTER, GetMethod, Method};
use thirtyfour::error::WebDriverError;

use crate::{Engine, EngineResult};

const IS_DISPLAYED_ERROR: &'static str = "Error while evaluating is displayed element action";
const GET_ATTRIBUTE_ERROR: &'static str = "Error while evaluating get attribute action";
const GETTER_ERROR: &'static str = "Failed to evaluate getter action";

impl<'a> Engine<'a> {
    pub async fn getter(&mut self, teststep: &Teststep) -> EngineResult<IdentifierValue> {
        if let Method::GETTER(method) = &teststep.get_method() {
            use GETTER::*;
            match method {
                GET_ATTRIBUTE => self.GET_ATTRIBUTE(teststep).await,
                IS_DISPLAYED => self.IS_DISPLAYED(teststep).await,
                IS_ENABLED => self.IS_ENABLED(teststep).await,
                IS_SELECTED => self.IS_SELECTED(teststep).await,
                GET_TITLE => self.GET_TITLE(teststep).await,
                GET_CURRENT_URL => self.GET_CURRENT_URL(teststep).await,
            }
        } else {
            Err(WebDriverError::FatalError(GETTER_ERROR.to_string()))
        }
    }

    pub async fn GET_ATTRIBUTE(&mut self, _body: &Teststep) -> EngineResult<IdentifierValue> {
        if let Teststep::Getter(_) = _body {
            let locator = self.get_locator(_body).await?;
            let attribute = self.get_attribute(_body).await?;
            let element = self.driver.find(locator).await?;
            let attr = element.attr(attribute).await?;
            Ok(IdentifierValue::String(attr))
        } else {
            Err(WebDriverError::FatalError(GET_ATTRIBUTE_ERROR.to_string()))
        }
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

    pub async fn IS_ENABLED(&mut self, _step: &Teststep) -> EngineResult<IdentifierValue> {
        let Teststep::Getter(_) = _step else {
            return Err(WebDriverError::FatalError(IS_DISPLAYED_ERROR.to_string()));
        };
        let locator = self.get_locator(_step).await?;
        let element = self.driver.find(locator).await?;
        let enabled = element.is_enabled().await?;
        Ok(IdentifierValue::Boolean(Some(enabled)))
    }

    pub async fn IS_SELECTED(&mut self, _step: &Teststep) -> EngineResult<IdentifierValue> {
        let Teststep::Getter(_) = _step else {
            return Err(WebDriverError::FatalError(IS_DISPLAYED_ERROR.to_string()));
        };
        let locator = self.get_locator(_step).await?;
        let element = self.driver.find(locator).await?;
        let selected = element.is_selected().await?;
        Ok(IdentifierValue::Boolean(Some(selected)))
    }

    pub async fn GET_CURRENT_URL(&mut self, _body: &Teststep) -> EngineResult<IdentifierValue> {
        let url = self.driver.current_url().await?;
        Ok(IdentifierValue::String(Some(url.to_string())))
    }
    pub async fn GET_TITLE(&mut self, _body: &Teststep) -> EngineResult<IdentifierValue> {
        let title = self.driver.title().await?;
        Ok(IdentifierValue::String(Some(title)))
    }
}
