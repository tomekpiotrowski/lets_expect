use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use colored::Colorize;
use lets_expect_core::assertions::{
    assertion_error::AssertionError, assertion_result::AssertionResult,
};

pub fn be_greater_than<R>(expected: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd,
{
    move |received| {
        if expected < *received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be greater than {}",
                    received, expected
                )],
            })
        }
    }
}

pub fn be_greater_or_equal_to<R>(expected: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd,
{
    move |received| {
        if expected <= *received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be greater or equal to {}",
                    received, expected
                )],
            })
        }
    }
}

pub fn be_less_than<R>(expected: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd,
{
    move |received| {
        if expected > *received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be less than {}",
                    received, expected
                )],
            })
        }
    }
}

pub fn be_less_or_equal_to<R>(expected: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd,
{
    move |received| {
        if expected >= *received {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be less or equal to {}",
                    received, expected
                )],
            })
        }
    }
}

pub fn be_between<R>(lower: R, upper: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd,
{
    move |received| {
        if lower <= *received && *received <= upper {
            Ok(())
        } else {
            let lower = format!("{:?}", lower).green().bold();
            let upper = format!("{:?}", upper).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be between {} and {}",
                    received, lower, upper
                )],
            })
        }
    }
}

pub fn be_close_to<R>(expected: R, delta: R) -> impl Fn(&R) -> AssertionResult
where
    R: Debug + PartialOrd + Sub<Output = R> + Add<Output = R> + Clone,
{
    move |received| {
        if expected.clone() - delta.clone() <= *received
            && *received <= expected.clone() + delta.clone()
        {
            Ok(())
        } else {
            let expected = format!("{:?}", expected).green().bold();
            let delta = format!("{:?}", delta).green().bold();
            let received = format!("{:?}", received).red().bold();
            Err(AssertionError {
                message: vec![format!(
                    "Expected {} to be close to {} with a delta of {}",
                    received, expected, delta
                )],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use colored::control::set_override;

    use crate::expected_err::expected_err;

    use super::*;

    #[test]
    fn be_greater_than_ok() {
        assert_eq!(be_greater_than(1)(&2), Ok(()));
    }

    #[test]
    fn be_greater_than_err() {
        set_override(false);
        assert_eq!(
            be_greater_than(2)(&1),
            expected_err(vec!["Expected 1 to be greater than 2"])
        );
    }

    #[test]
    fn be_greater_or_equal_to_ok() {
        assert_eq!(be_greater_or_equal_to(1)(&1), Ok(()));
    }

    #[test]
    fn be_greater_or_equal_to_err() {
        set_override(false);
        assert_eq!(
            be_greater_or_equal_to(2)(&1),
            expected_err(vec!["Expected 1 to be greater or equal to 2"])
        );
    }

    #[test]
    fn be_less_than_ok() {
        assert_eq!(be_less_than(2)(&1), Ok(()));
    }

    #[test]
    fn be_less_than_err() {
        set_override(false);
        assert_eq!(
            be_less_than(1)(&2),
            expected_err(vec!["Expected 2 to be less than 1"])
        );
    }

    #[test]
    fn be_less_or_equal_to_ok() {
        assert_eq!(be_less_or_equal_to(2)(&2), Ok(()));
    }

    #[test]
    fn be_less_or_equal_to_err() {
        set_override(false);
        assert_eq!(
            be_less_or_equal_to(1)(&2),
            expected_err(vec!["Expected 2 to be less or equal to 1"])
        );
    }

    #[test]
    fn be_between_ok() {
        assert_eq!(be_between(1, 3)(&2), Ok(()));
    }

    #[test]
    fn be_between_err() {
        set_override(false);
        assert_eq!(
            be_between(1, 3)(&4),
            expected_err(vec!["Expected 4 to be between 1 and 3"])
        );
    }

    #[test]
    fn be_close_to_ok() {
        assert_eq!(be_close_to(1f32, 0.5)(&1.5), Ok(()));
    }

    #[test]
    fn be_close_to_err() {
        set_override(false);
        assert_eq!(
            be_close_to(1f64, 0.5)(&2.5),
            expected_err(vec!["Expected 2.5 to be close to 1.0 with a delta of 0.5"])
        );
    }
}
