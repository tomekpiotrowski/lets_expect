use colored::Colorize;
use lets_expect_core::assertions::{
    assertion_error::AssertionError, assertion_result::AssertionResult,
};
use std::fmt::Debug;

pub fn be_ok<R, E>(value: &Result<R, E>) -> AssertionResult
where
    R: Debug,
    E: Debug,
{
    if value.is_ok() {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError {
            message: vec![format!("Expected {} to be Ok", value)],
        })
    }
}

pub fn be_err<R, E>(value: &Result<R, E>) -> AssertionResult
where
    R: Debug,
    E: Debug,
{
    if value.is_err() {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError {
            message: vec![format!("Expected {} to be Err", value)],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected_err::expected_err;
    use colored::control::set_override;

    #[test]
    fn test_be_ok_ok() {
        assert_eq!(be_ok(&(Ok(1) as Result<i32, i32>)), Ok(()));
    }

    #[test]
    fn test_be_ok_err() {
        set_override(false);
        assert_eq!(
            be_ok(&(Err(1) as Result<i32, i32>)),
            expected_err(vec!["Expected Err(1) to be Ok"])
        );
    }

    #[test]
    fn test_be_err_ok() {
        assert_eq!(be_err(&(Err(1) as Result<i32, i32>)), Ok(()));
    }

    #[test]
    fn test_be_err_err() {
        set_override(false);
        assert_eq!(
            be_err(&(Ok(1) as Result<i32, i32>)),
            expected_err(vec!["Expected Ok(1) to be Err"])
        );
    }
}
