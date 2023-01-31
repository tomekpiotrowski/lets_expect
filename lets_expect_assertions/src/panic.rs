use lets_expect_core::assertions::{
    assertion_error::AssertionError, assertion_result::AssertionResult,
};
use std::any::Any;

pub fn panic<R>(result: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match result {
        Ok(_) => Err(AssertionError {
            message: vec![format!("Expected subject to panic, but it didn't")],
        }),
        Err(_) => Ok(()),
    }
}

pub fn not_panic<R>(result: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(AssertionError {
            message: vec![format!("Expected subject to not panic, but it did")],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected_err::expected_err;
    use colored::control::set_override;

    #[test]
    fn test_panic_ok() {
        let result: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        assert_eq!(panic(&result), Ok(()));
    }

    #[test]
    fn test_panic_err() {
        let result: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        set_override(false);
        assert_eq!(
            panic(&result),
            expected_err(vec!["Expected subject to panic, but it didn't"])
        );
    }

    #[test]
    fn test_not_panic_ok() {
        let result: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        assert_eq!(not_panic(&result), Ok(()));
    }

    #[test]
    fn test_not_panic_err() {
        let result: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        set_override(false);
        assert_eq!(
            not_panic(&result),
            expected_err(vec!["Expected subject to not panic, but it did"])
        );
    }
}
