use thirtyfour::{error::WebDriverError, WebDriver};

use crate::{
    ast::{
        identifier_value::IdentifierValue,
        testcase::TestCase,
        testcase_body::{GetMethod, TestcaseBody},
        var_decl::VarRHS,
    },
    class::{CustomEngine, ElementEngine, Method, CUSTOM, ELEMENT},
    engine::{element::Element, EngineResult},
};

pub struct Custom<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Custom<'a> {
    pub async fn new(
        driver: &WebDriver,
        body: &TestcaseBody,
        testcase: &mut TestCase,
    ) -> EngineResult<()> {
        let custom = Custom { driver };
        if let Method::CUSTOM(method) = body.get_method() {
            match method {
                CUSTOM::VAR_DECLARATION => custom.VAR_DECLARATION(body, testcase).await?,
            };
        }
        Ok(())
    }
}

impl<'a> CustomEngine for Custom<'a> {
    async fn VAR_DECLARATION(
        &self,
        _body: &TestcaseBody,
        _testcase: &mut TestCase,
    ) -> EngineResult<()> {
        if let TestcaseBody::VAR_DECL(step) = _body {
            match &step.rhs {
                VarRHS::Getter(getter) => {
                    match getter.get_method() {
                        Method::ELEMENT(ELEMENT::GET_ATTRIBUTE) => {
                            let element = Element {
                                driver: &self.driver,
                            };
                            let attribute_value = element
                                .GET_ATTRIBUTE(&TestcaseBody::GETTER(getter.clone()))
                                .await?;
                            _testcase.insert_variable_value(
                                step.name.clone(),
                                IdentifierValue::String(attribute_value),
                            );
                        }
                        _ => {
                            println!("Note yet handled");
                        }
                    };
                }
                VarRHS::String(_) => {
                    todo!();
                }
                VarRHS::Var(_) => {
                    todo!();
                }
            }
        }
        Ok(())
    }
}
