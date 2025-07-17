use std::collections::HashMap;

use crate::{
    ast::{
        if_stmt::IfStmt,
        teststep::TestStep,
        var_decl::{IdentifierValue, VarDecl},
    },
    enums::CapabilityValue,
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestCase {
    capabilities: HashMap<String, CapabilityValue>,
    variables: HashMap<String, IdentifierValue>,
    test_steps: Vec<TestcaseBody>,
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

    pub fn insert_variable(&mut self, var: &VarDecl) {
        self.variables.insert(var.name.clone(), var.value.clone());
    }

    pub fn insert_teststep(&mut self, test_step: TestStep) {
        self.test_steps.push(TestcaseBody::TESTSTEP(test_step));
    }

    pub fn get_teststeps(&self) -> &Vec<TestStep> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum TestcaseBody {
    TESTSTEP(TestStep),
    IF(IfStmt),
    VarDecl(),
}
