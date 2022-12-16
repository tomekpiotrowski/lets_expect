use super::executed_test_case::ExecutedTestCase;
use std::fmt::Debug;

pub struct TestFailure {
    test_cases: Vec<ExecutedTestCase>,
}

impl TestFailure {
    pub fn new(test_cases: Vec<ExecutedTestCase>) -> Self {
        Self { test_cases }
    }
}

impl Debug for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let test_cases = self.test_cases.iter().map(|test_case| test_case.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "\n\n{}", test_cases)
    }
}