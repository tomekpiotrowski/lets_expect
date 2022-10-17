use colored::Colorize;
use lets_expect_core::assertions::{assertion_error::AssertionError, assertion_result::AssertionResult};

pub fn be_true(value: bool) -> AssertionResult {
    if value {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError { message: vec![format!("Expected {} to be true", value)] })
    }
}

pub fn be_false(value: bool) -> AssertionResult {
    if !value {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError { message: vec![format!("Expected {} to be false", value)] })
    }
}

#[cfg(test)]
mod test_super {
    use colored::control::set_override;
    use crate::expected_err::expected_err;
    use super::*;

    #[test]
    fn test_be_true_ok() {
        assert_eq!(be_true(true), Ok(()));
    }

    #[test]
    fn test_be_true_err() {
        set_override(false);
        assert_eq!(be_true(false), expected_err(vec!["Expected false to be true"]));
    }

    #[test]
    fn test_be_false() {
        assert_eq!(be_false(false), Ok(()));
    }

    #[test]
    fn test_be_false_err() {
        set_override(false);
        assert_eq!(be_false(true), expected_err(vec!["Expected true to be false"]));
    }
}