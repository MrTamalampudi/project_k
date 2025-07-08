use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    test_steps: Vec<Rc<RefCell<TestcaseBody>>>,
    pub test_step: Option<Rc<RefCell<TestcaseBody>>>,
}

impl TestCase {
    pub fn new() -> TestCase {
        TestCase {
            capabilities: HashMap::new(),
            variables: HashMap::new(),
            test_steps: vec![],
            test_step: None,
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

    pub fn insert_teststep(&mut self, body: Rc<RefCell<TestcaseBody>>) {
        self.test_steps.push(body);
    }

    pub fn get_last_teststep_entry(&mut self) -> Option<Rc<RefCell<TestcaseBody>>> {
        self.test_steps.last().cloned()
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
