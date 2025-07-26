use thirtyfour::WebDriver;

use crate::{
    ast::{
        getter::Getter,
        testcase_body::{GetMethod, TestcaseBody},
        var_decl::VarRHS,
    },
    class::{CustomEngine, ElementEngine, Method, CUSTOM, ELEMENT},
    engine::element::Element,
};

pub struct Custom<'a> {
    pub driver: &'a WebDriver,
}

impl<'a> Custom<'a> {
    pub async fn new(driver: &WebDriver, body: &TestcaseBody) {
        let custom = Custom { driver };
        if let Method::CUSTOM(method) = body.get_method() {
            match method {
                CUSTOM::VAR_DECLARATION => custom.VAR_DECLARATION(body).await,
            };
        }
    }
}

impl<'a> CustomEngine for Custom<'a> {
    async fn VAR_DECLARATION(&self, _body: &TestcaseBody) -> () {
        if let TestcaseBody::VAR_DECL(step) = _body {
            match &step.rhs {
                VarRHS::Getter(getter) => {
                    match getter.get_method() {
                        Method::ELEMENT(ELEMENT::GET_ATTRIBUTE) => {
                            let element = Element {
                                driver: &self.driver,
                            };
                            let stri = element
                                .GET_ATTRIBUTE(&TestcaseBody::GETTER(getter.clone()))
                                .await;
                            println!("---------- {}", stri);
                        }
                        _ => {
                            println!("Note yet handled");
                        }
                    };
                }
                VarRHS::String(string) => {
                    todo!();
                }
                VarRHS::Var(var) => {
                    todo!();
                }
            }
        }
    }
}
