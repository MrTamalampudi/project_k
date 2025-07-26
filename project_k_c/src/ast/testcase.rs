use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        identifier_value::IdentifierValue, testcase_body::TestcaseBody, teststep::TestStep,
        var_decl::VarDecl,
    },
    enums::CapabilityValue,
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestCase {
    capabilities: HashMap<String, CapabilityValue>,
    pub variables: HashMap<String, IdentifierValue>,
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

    pub fn insert_variable(&mut self, var: VarDecl) {
        self.variables
            .insert(var.name.clone(), var.type_.to_identifier_value());
    }

    pub fn insert_teststep(&mut self, body: TestcaseBody) {
        let teststep_refcell = RefCell::new(body);
        let teststep_rc = Rc::new(teststep_refcell);
        if self.test_step.is_none() {
            self.test_step = Some(teststep_rc.clone());
        } else {
            let teststep = self.get_last_teststep_entry().unwrap();
            let teststep_deref = &mut *teststep.borrow_mut();
            if let TestcaseBody::TESTSTEP(step) = teststep_deref {
                step.next = Some(teststep_rc.clone());
            }
        }
        self.test_steps.push(teststep_rc);
    }

    pub fn get_last_teststep_entry(&mut self) -> Option<Rc<RefCell<TestcaseBody>>> {
        self.test_steps.last().cloned()
    }

    pub fn get_teststeps(&self) -> &Vec<TestStep> {
        todo!()
    }
}
