use std::collections::HashMap;

use crate::{
    ast::{if_stmt::IfStmt, teststep::TestStep},
    enums::{CapabilityValue, IdentifierValue},
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestCase {
    capabilities: HashMap<String, CapabilityValue>,
    variables: HashMap<String, IdentifierValue>,
    test_steps: Vec<TestStep>,
}

impl TestCase {
    pub fn new() -> TestCase {
        TestCase {
            capabilities: HashMap::new(),
            variables: HashMap::new(),
            test_steps: vec![],
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

    pub fn insert_variable(&mut self, identifier: &String, value: &IdentifierValue) {
        self.variables.insert(identifier.clone(), value.clone());
    }

    pub fn insert_teststep(&mut self, test_step: TestStep) {
        self.test_steps.push(test_step);
    }

    pub fn get_teststeps(&self) -> &Vec<TestStep> {
        &self.test_steps
    }
}

pub enum TestcaseBody {
    TESTSTEP(TestStep),
    IF(IfStmt),
}
