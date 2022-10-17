use lets_expect_core::assertions::{assertion_result::AssertionResult, assertion_error::AssertionError};

pub(super) fn expected_err(lines: Vec<&str>) -> AssertionResult {
    let message = lines
        .into_iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    Err(AssertionError::new(message))
}