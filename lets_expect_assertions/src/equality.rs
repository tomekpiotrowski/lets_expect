use std::fmt::Debug;
use colored::Colorize;
use lets_expect_core::assertions::{assertion_error::AssertionError, assertion_result::AssertionResult};


pub fn equal<R>(expected: R) -> impl Fn(R) -> AssertionResult where R: Debug + PartialEq {
    move |received| {
        if expected == received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError { message: vec![format!("Expected: {}", expected), format!("Received: {}", received)] })
        }
    }
}

pub fn not_equal<R>(expected: R) -> impl Fn(R) -> AssertionResult where R: Debug + PartialEq {
    move |received| {
        if expected != received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            Err(AssertionError { message: vec![format!("Expected something else than {}", expected)] })
        }
    }
}

#[cfg(test)]
mod tests {
    use colored::control::set_override;
    use crate::expected_err::expected_err;
    use super::*;

    #[test]
    fn test_equal_ok() {
        assert_eq!(equal(1)(1), Ok(()));
    }

    #[test]
    fn test_equal_err() {
        set_override(false);
        assert_eq!(equal(1)(2), expected_err(vec!["Expected: 1", "Received: 2"]));
    }

    #[test]
    fn test_not_equal_ok() {
        assert_eq!(not_equal(1)(2), Ok(()));
    }

    #[test]
    fn test_not_equal_err() {
        set_override(false);
        assert_eq!(not_equal(1)(1), expected_err(vec!["Expected something else than 1"]));
    }
}