use thirtyfour::error::WebDriverError;
use thirtyfour::By;

use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, EXPR_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::identifier_value::IdentifierValue;
use crate::ast::teststep::GetMethod;
use crate::ast::teststep::Teststep;
use crate::class::{ElementEngine, Method, ELEMENT};
use crate::engine::errors::{INVALID_INPUT, INVALID_LOC_EXPR};
use crate::engine::{Engine, EngineResult};
use crate::parser::locator::LocatorStrategy;

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
            };
        };
        Ok(())
    }
}

impl<'a> ElementEngine for Engine<'a> {
    async fn CLEAR(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SUBMIT(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
    async fn SENDKEYS(&mut self, _step: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(action) = _step {
            let locator_arg = action.arguments.get(LOCATOR_ARGKEY).unwrap();
            let input_arg = action.arguments.get(EXPR_ARGKEY).unwrap();
            let locator = match locator_arg {
                Args::Locator(loc) => loc.to_by(),
                Args::Expr(expr) => match self.locator_eval(expr).await {
                    Ok(loc) => loc,
                    Err(err) => return Err(WebDriverError::FatalError(err)),
                },
                _ => return Err(WebDriverError::FatalError(INVALID_LOC_EXPR.to_string())),
            };
            let input = match input_arg {
                Args::Number(num) => num.to_string(),
                Args::String(str) => str.to_string(),
                Args::Expr(expr) => match self.input_eval(expr).await {
                    Ok(val) => val,
                    Err(err) => return Err(WebDriverError::FatalError(err)),
                },
                _ => return Err(WebDriverError::FatalError(INVALID_INPUT.to_string())),
            };

            if let Ok(element) = self.driver.find(locator).await {
                element.send_keys(input).await?
            }
        }
        Ok(())
    }
    async fn GET_ATTRIBUTE(&mut self, _body: &Teststep) -> EngineResult<Option<String>> {
        if let Teststep::Getter(getter) = _body {
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

    async fn CLICK(&mut self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::Action(step) = _body {
            match step.arguments.get(LOCATOR_ARGKEY).unwrap() {
                Args::Locator(locator) => {
                    let by = locator.to_by();
                    let element = self.driver.find(by).await?;
                    element.click().await?;
                }
                Args::Expr(expr) => {
                    let value = self.eval(expr).await.ok().unwrap();
                    if let IdentifierValue::String(string) = value {
                        let vv = LocatorStrategy::parse(&string.unwrap());
                        let by = vv.to_by();
                        let element = self.driver.find(by).await?;
                        element.click().await?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
