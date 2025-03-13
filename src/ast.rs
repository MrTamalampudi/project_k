use crate::{
    actions::{Action, ActionOption},
    enums::{CapabilityValue, IdentifierValue},
};
use std::{
    collections::HashMap,
    fmt::{self},
};

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line:{} column:{}", self.line, self.column,)
    }
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }

    pub fn next_column(&mut self) -> Self {
        self.column += 1;
        *self
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TestStep {
    start: Location,
    end: Location,
    pub action: Action,
    option: ActionOption,
    pub arguments: Vec<String>,
}

impl TestStep {
    pub fn new(
        start: Location,
        end: Location,
        action: Action,
        option: ActionOption,
        arguments: Vec<String>,
    ) -> TestStep {
        TestStep {
            start,
            end,
            action,
            option,
            arguments,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Testcase {
    capabilities: HashMap<String, CapabilityValue>,
    variables: HashMap<String, IdentifierValue>,
    prerequisites: Vec<Testcase>,
    test_steps: Vec<TestStep>,
}

impl Testcase {
    pub fn init() -> Testcase {
        Testcase {
            capabilities: HashMap::new(),
            variables: HashMap::new(),
            prerequisites: vec![],
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

    pub fn get_prerequisite(&self) -> &Vec<Testcase> {
        &self.prerequisites
    }

    pub fn insert_prerequisite(&mut self, testcase: Testcase) {
        self.prerequisites.push(testcase);
    }
}
