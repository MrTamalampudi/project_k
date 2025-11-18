use std::collections::HashMap;

use crate::{
    ast::{
        action::Action, identifier_value::IdentifierValue, teststep::Teststep, var_decl::VarDecl,
    },
    enums::CapabilityValue,
};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct TestCase {
    capabilities: HashMap<String, CapabilityValue>,
    pub variables: HashMap<String, IdentifierValue>,
    pub body: Vec<Teststep>,
}

impl TestCase {
    pub fn new() -> TestCase {
        TestCase {
            capabilities: HashMap::new(),
            variables: HashMap::new(),
            body: vec![],
        }
    }

    pub fn get_capability(&self, capability: &String) -> CapabilityValue {
        self.capabilities
            .get(capability)
            .map_or(CapabilityValue::NONE, |capability_value| {
                capability_value.clone()
            })
    }

    pub fn insert_capability(&mut self, capability: &String, value: &CapabilityValue) {
        self.capabilities.insert(capability.clone(), value.clone());
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

    pub fn insert_teststep(&mut self, teststep: Teststep) {
        self.body.push(teststep);
    }

    pub fn get_teststeps(&self) -> &Vec<Action> {
        todo!()
    }
}
