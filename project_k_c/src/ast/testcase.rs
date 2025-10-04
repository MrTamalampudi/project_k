use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    test_steps: Vec<Rc<RefCell<Teststep>>>,
    pub test_step: Option<Rc<RefCell<Teststep>>>,
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

    pub fn insert_variable_value(&mut self, ident: String, value: IdentifierValue) {
        let variable = self.variables.get(&ident);
        if let Some(val) = variable {
            if val.matches(&value) {
                self.variables.insert(ident, value);
            }
        }
    }

    pub fn insert_teststep(&mut self, body: Teststep) {
        let teststep_refcell = RefCell::new(body);
        let teststep_rc = Rc::new(teststep_refcell);
        if self.test_step.is_none() {
            self.test_step = Some(teststep_rc.clone());
        } else {
            let teststep = self.get_last_teststep_entry().unwrap();
            let teststep_deref = &mut *teststep.borrow_mut();
            if let Teststep::Action(step) = teststep_deref {
                step.next = Some(teststep_rc.clone());
            }
            if let Teststep::VarDecl(step) = teststep_deref {
                step.next = Some(teststep_rc.clone());
            }
        }
        self.test_steps.push(teststep_rc);
    }

    pub fn get_last_teststep_entry(&mut self) -> Option<Rc<RefCell<Teststep>>> {
        self.test_steps.last().cloned()
    }

    pub fn get_teststeps(&self) -> &Vec<Action> {
        todo!()
    }
}
