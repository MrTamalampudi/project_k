use crate::ast::arguments::{Args, ATTRIBUTE_ARGKEY, LOCATOR_ARGKEY};
use crate::ast::teststep::GetMethod;
use crate::ast::teststep::Teststep;
use crate::class::{ElementEngine, Method, ELEMENT};
use crate::engine::{Engine, EngineResult};

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
            if let Args::Locator(locator) = step.arguments.get(LOCATOR_ARGKEY).unwrap() {
                let by = locator.to_by();
                let element = self.driver.find(by).await?;
                element.click().await?;
            }
        }
        Ok(())
    }
}
