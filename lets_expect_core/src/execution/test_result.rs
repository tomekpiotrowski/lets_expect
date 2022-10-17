use super::{test_failure::TestFailure, executed_test_case::ExecutedTestCase};

pub type TestResult = Result<(), TestFailure>;

pub fn test_result_from_cases(test_cases: Vec<ExecutedTestCase>) -> TestResult {
    if test_cases.iter().any(|test_case| test_case.failed()) {
        Err(TestFailure::new(test_cases))
    } else {
        Ok(())
    }
}