use std::collections::HashMap;

use crate::{action::Action, identifier_value::IdentifierValue, teststep::Body, var_decl::VarDecl};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct TestCase {
    pub variables: HashMap<String, IdentifierValue>,
    pub body: Body,
}

impl TestCase {
    pub fn new() -> TestCase {
        TestCase {
            variables: HashMap::new(),
            body: Body::new(),
        }
    }

    pub fn insert_variable(&mut self, var: VarDecl) {
        self.variables
            .insert(var.name.clone(), var.type_.to_identifier_value());
    }

    pub fn insert_variable_value(&mut self, ident: String, value: IdentifierValue) {
        let variable = self.variables.get(&ident);
        if let Some(val) = variable {
            if val.matches(&value) {
                self.variables.insert(ident, value);
            }
        }
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    pub fn get_teststeps(&self) -> &Vec<Action> {
        todo!()
    }
}
