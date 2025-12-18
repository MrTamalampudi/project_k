use ast::testcase::TestCase;

#[derive(Debug, Clone)]
pub struct Program {
    pub testcase: TestCase,
}

impl Program {
    pub fn new() -> Program {
        Program {
            testcase: TestCase::new(),
        }
    }
}
