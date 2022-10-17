use std::any::Any;
use lets_expect_core::assertions::{assertion_error::AssertionError, assertion_result::AssertionResult};

pub fn panic<R>(result: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match result {
        Ok(_) => Err(AssertionError { message: vec![format!("Expected subject to panic, but it didn't")] }),
        Err(_) => Ok(())
    }
}

pub fn not_panic<R>(result: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(AssertionError { message: vec![format!("Expected subject to not panic, but it did")] })
    }
}

pub fn from_panic<R>(before: &Result<R, Box<dyn Any + Send>>, _after: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match before {
        Ok(_) => Err(AssertionError { message: vec![format!("Expected subject to panic, but it didn't")] }),
        Err(_) => Ok(())
    }
}

pub fn from_not_panic<R>(before: &Result<R, Box<dyn Any + Send>>, _after: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match before {
        Ok(_) => Ok(()),
        Err(_) => Err(AssertionError { message: vec![format!("Expected subject to not panic, but it did")] })
    }
}

pub fn to_panic<R>(_before: &Result<R, Box<dyn Any + Send>>, after: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match after {
        Ok(_) => Err(AssertionError { message: vec![format!("Expected subject to panic, but it didn't")] }),
        Err(_) => Ok(())
    }
}

pub fn to_not_panic<R>(_before: &Result<R, Box<dyn Any + Send>>, after: &Result<R, Box<dyn Any + Send>>) -> AssertionResult {
    match after {
        Ok(_) => Ok(()),
        Err(_) => Err(AssertionError { message: vec![format!("Expected subject to not panic, but it did")] })
    }
}

#[cfg(test)]
mod tests {
    use colored::control::set_override;
    use crate::expected_err::expected_err;
    use super::*;

    #[test]
    fn test_panic_ok() {
        let result: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        assert_eq!(panic(&result), Ok(()));
    }

    #[test]
    fn test_panic_err() {
        let result: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        set_override(false);
        assert_eq!(panic(&result), expected_err(vec!["Expected subject to panic, but it didn't"]));
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
        assert_eq!(not_panic(&result), expected_err(vec!["Expected subject to not panic, but it did"]));
    }

    #[test]
    fn test_from_panic_ok() {
        let before: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        let after: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        assert_eq!(from_panic(&before, &after), Ok(()));
    }

    #[test]
    fn test_from_panic_err() {
        let before: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        let after: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        set_override(false);
        assert_eq!(from_panic(&before, &after), expected_err(vec!["Expected subject to panic, but it didn't"]));
    }

    #[test]
    fn test_from_not_panic_ok() {
        let before: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        let after: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        assert_eq!(from_not_panic(&before, &after), Ok(()));
    }

    #[test]
    fn test_from_not_panic_err() {
        let before: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        let after: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        set_override(false);
        assert_eq!(from_not_panic(&before, &after), expected_err(vec!["Expected subject to not panic, but it did"]));
    }

    #[test]
    fn test_to_panic_ok() {
        let before: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        let after: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        assert_eq!(to_panic(&before, &after), Ok(()));
    }

    #[test]
    fn test_to_panic_err() {
        let before: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        let after: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        set_override(false);
        assert_eq!(to_panic(&before, &after), expected_err(vec!["Expected subject to panic, but it didn't"]));
    }

    #[test]
    fn test_to_not_panic_ok() {
        let before: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        let after: Result<String, Box<dyn Any + Send>> = Ok("test".to_string());
        assert_eq!(to_not_panic(&before, &after), Ok(()));
    }

    #[test]
    fn test_to_not_panic_err() {
        let before: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        let after: Result<String, Box<dyn Any + Send>> = Err(Box::new("test"));
        set_override(false);
        assert_eq!(to_not_panic(&before, &after), expected_err(vec!["Expected subject to not panic, but it did"]));
    }

}