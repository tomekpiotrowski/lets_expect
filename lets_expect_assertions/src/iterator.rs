use std::slice::Iter;

use lets_expect_core::assertions::{
    assertion_error::AssertionError, assertion_result::AssertionResult,
};

pub fn all<R>(
    assertion: impl Fn(&R) -> AssertionResult,
) -> impl FnOnce(&mut Iter<R>) -> AssertionResult {
    move |iter| {
        let results: Vec<AssertionResult> = iter.map(assertion).collect();

        if results.iter().all(|result| result.is_ok()) {
            Ok(())
        } else {
            let mut errors = vec![];
            for result in results {
                if let Err(err) = result {
                    errors.push(err.message.join(" "));
                }
            }
            Err(AssertionError::new(errors))
        }
    }
}

pub fn any<R>(
    assertion: impl Fn(&R) -> AssertionResult,
) -> impl FnOnce(&mut dyn Iterator<Item = &R>) -> AssertionResult {
    move |iter| {
        let results: Vec<AssertionResult> = iter.map(assertion).collect();

        if results.iter().any(|result| result.is_ok()) {
            Ok(())
        } else {
            let mut errors = vec![];
            for result in results {
                if let Err(err) = result {
                    errors.push(err.message.join(" "));
                }
            }
            Err(AssertionError::new(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ones() {
        assert!(all(|&x| if x == 1 {
            Ok(())
        } else {
            Err(AssertionError::new(vec!["err".to_string()]))
        })(&mut vec![1, 1, 1].iter())
        .is_ok());
    }

    #[test]
    fn test_not_all_ones() {
        assert!(all(|&x| if x == 1 {
            Ok(())
        } else {
            Err(AssertionError::new(vec!["err".to_string()]))
        })(&mut vec![1, 2, 1].iter())
        .is_err());
    }

    #[test]
    fn test_any_success() {
        assert!(any(|&x| if x == 1 {
            Ok(())
        } else {
            Err(AssertionError::new(vec!["err".to_string()]))
        })(&mut vec![1, 0, 0].iter())
        .is_ok());
    }

    #[test]
    fn test_any_failure() {
        assert!(any(|&x| if x == 1 {
            Ok(())
        } else {
            Err(AssertionError::new(vec!["err".to_string()]))
        })(&mut vec![0, 0, 0].iter())
        .is_ok());
    }
}
