use colored::Colorize;

use lets_expect_core::assertions::{assertion_error::AssertionError, assertion_result::AssertionResult};
use std::{fmt::Debug, ops::Sub};

pub fn from<R>(expected: R) -> impl Fn(R, R) -> AssertionResult where R: Debug + PartialEq {
    move |from, _| {
        if expected == from {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let from = format!("{:?}", from).red().bold();
            Err(AssertionError { message: vec![format!("Expected to change from {}, but it was {} instead", expected, from)] })
        }
    }
}

pub fn to<R>(expected: R) -> impl Fn(R, R) -> AssertionResult where R: Debug + PartialEq {
    move |_, to| {
        if expected == to {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let to = format!("{:?}", to).red().bold();
            Err(AssertionError { message: vec![format!("Expected to change to {}, but it was {} instead", expected, to)] })
        }
    }
}

pub fn by<R>(expected: R) -> impl Fn(R, R) -> AssertionResult where R: Debug + PartialEq + Sub<Output = R> + Clone {
    move |from, to| {
        let diff = to - from;
        if expected == diff {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let diff = format!("{:?}", diff).red().bold();
            Err(AssertionError { message: vec![format!("Expected to change by {}, but it was changed by {} instead", expected, diff)] })
        }
    }
}

#[cfg(test)]
mod tests {
    use colored::control::set_override;
    use crate::expected_err::expected_err;
    use super::*;

    #[test]
    fn test_from_ok() {
        assert_eq!(from(1)(1, 2), Ok(()));
    }

    #[test]
    fn test_from_err() {
        set_override(false);
        assert_eq!(from(1)(2, 2), expected_err(vec!["Expected to change from 1, but it was 2 instead"]));
    }

    #[test]
    fn test_to_ok() {
        assert_eq!(to(2)(1, 2), Ok(()));
    }

    #[test]
    fn test_to_err() {
        set_override(false);
        assert_eq!(to(2)(1, 1), expected_err(vec!["Expected to change to 2, but it was 1 instead"]));
    }

    #[test]
    fn test_by_ok() {
        assert_eq!(by(1)(1, 2), Ok(()));
    }

    #[test]
    fn test_by_err() {
        set_override(false);
        assert_eq!(by(1)(1, 1), expected_err(vec!["Expected to change by 1, but it was changed by 0 instead"]));
    }
}