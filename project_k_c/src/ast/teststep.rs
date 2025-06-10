use crate::{
    actions::{Action, ActionOption},
    location::Location,
};

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
