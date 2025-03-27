use crate::{
    actions::{Action, ActionOption},
    enums::{CapabilityValue, IdentifierValue},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self},
    rc::Rc,
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

    pub fn get_location(&self) -> Location {
        self.clone()
    }

    pub fn dummy() -> Location {
        Location { line: 0, column: 0 }
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
pub struct TestCase {
    capabilities: HashMap<String, CapabilityValue>,
    variables: HashMap<String, IdentifierValue>,
    prerequisites: Vec<Rc<RefCell<TestCase>>>,
    test_steps: Vec<TestStep>,
}

impl<'a> TestCase {
    pub fn init() -> TestCase {
        TestCase {
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

    pub fn get_prerequisite(&self) -> &Vec<Rc<RefCell<TestCase>>> {
        &self.prerequisites
    }

    pub fn insert_prerequisite(&mut self, testcase: Rc<RefCell<TestCase>>) {
        self.prerequisites.push(testcase);
    }
}

#[derive(Debug, Clone)]
pub struct TestSuite {
    testcases: Vec<TestCase>,
}

#[derive(Debug, Clone)]
pub struct TestPlan {
    testsuites: Vec<TestSuite>,
}

impl TestPlan {
    pub fn new() -> TestPlan {
        TestPlan {
            testsuites: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryPoint {
    TESTCASE(Rc<RefCell<TestCase>>),
    TESTSUITE(TestSuite),
    TESTPLAN(TestPlan),
    NONE,
}

#[derive(Debug, Clone)]
pub struct Program {
    entrypoint: EntryPoint,
    testcases: Vec<Rc<RefCell<TestCase>>>,
    testsuites: Vec<TestSuite>,
    testplan: TestPlan,
}

impl Program {
    pub fn new() -> Program {
        Program {
            entrypoint: EntryPoint::NONE,
            testcases: Vec::new(),
            testsuites: Vec::new(),
            testplan: TestPlan::new(),
        }
    }

    pub fn push_testcase(&mut self, testcase: &TestCase) -> Rc<RefCell<TestCase>> {
        let ref_testcase = Rc::new(RefCell::new(testcase.clone()));
        self.testcases.push(Rc::clone(&ref_testcase));
        ref_testcase
    }

    pub fn set_entrypoint(&mut self, entrypoint: EntryPoint) {
        self.entrypoint = entrypoint;
    }

    pub fn get_tes(&self) {
        let mut t = self.testcases.get(0).expect("yes").borrow_mut();
        t.insert_variable(
            &"hi".to_string(),
            &IdentifierValue::STRING("check".to_string()),
        );
        println!("{:#?}", t);
    }
}
