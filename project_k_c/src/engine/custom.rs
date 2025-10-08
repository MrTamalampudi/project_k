use crate::{
    ast::{
        identifier_value::IdentifierValue,
        teststep::{GetMethod, Teststep},
        var_decl::VarRHS,
    },
    class::{CustomEngine, ElementEngine, Method, WebDriverEngine, ELEMENT, WEB_DRIVER},
    engine::{Engine, EngineResult},
};

impl<'a> CustomEngine for Engine<'a> {
    async fn VAR_DECLARATION(&mut self, _body: &Teststep) -> EngineResult<()> {
        if let Teststep::VarDecl(step) = _body {
            match &step.rhs {
                VarRHS::Getter(getter) => {
                    match getter.get_method() {
                        Method::ELEMENT(ELEMENT::GET_ATTRIBUTE) => {
                            let attribute_value = self
                                .GET_ATTRIBUTE(&Teststep::Getter(getter.clone()))
                                .await?;
                            self.testcase.insert_variable_value(
                                step.name.clone(),
                                IdentifierValue::String(attribute_value),
                            );
                        }
                        Method::WEB_DRIVER(WEB_DRIVER::GET_CURRENT_URL) => {
                            let url = self
                                .GET_CURRENT_URL(&Teststep::Getter(getter.clone()))
                                .await?;
                            self.testcase.insert_variable_value(
                                step.name.clone(),
                                IdentifierValue::String(url),
                            );
                        }
                        Method::WEB_DRIVER(WEB_DRIVER::GET_TITLE) => {
                            let title = self.GET_TITLE(&Teststep::Getter(getter.clone())).await?;
                            self.testcase.insert_variable_value(
                                step.name.clone(),
                                IdentifierValue::String(title),
                            );
                        }
                        _ => {
                            println!("Note yet handled");
                        }
                    };
                }
                VarRHS::Expression(expr) => {
                    let _ = self.eval(expr);
                }
            }
        }
        Ok(())
    }

    async fn ASSERT(&mut self, _step: &Teststep) -> EngineResult<()> {
        Ok(())
    }
}
