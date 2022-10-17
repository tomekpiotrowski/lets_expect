use colored::Colorize;
use lets_expect_core::assertions::{assertion_error::AssertionError, assertion_result::AssertionResult};
use std::fmt::Debug;

pub fn be_some<R>(value: Option<R>) -> AssertionResult where R: Debug {
    if value.is_some() {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError { message: vec![format!("Expected {} to be Some", value)] })
    }
}

pub fn be_none<R>(value: Option<R>) -> AssertionResult where R: Debug {
    if value.is_none() {
        Ok(())
    } else {
        let value = format!("{:?}", value).red().bold();
        Err(AssertionError { message: vec![format!("Expected {} to be None", value)] })
    }
}

#[cfg(test)]
mod tests {
    use colored::control::set_override;
    use crate::expected_err::expected_err;
    use super::*;

    #[test]
    fn test_be_some_ok() {
        assert_eq!(be_some(Some(1)), Ok(()));
    }

    #[test]
    fn test_be_some_err() {
        set_override(false);
        assert_eq!(be_some(None as Option<u8>), expected_err(vec!["Expected None to be Some"]));
    }

    #[test]
    fn test_be_none_ok() {
        assert_eq!(be_none(None as Option<u8>), Ok(()));
    }

    #[test]
    fn test_be_none_err() {
        set_override(false);
        assert_eq!(be_none(Some(1)), expected_err(vec!["Expected Some(1) to be None"]));
    }
}