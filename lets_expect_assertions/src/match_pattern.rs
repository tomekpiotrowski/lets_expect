pub use colored::Colorize;
pub use lets_expect_core::assertions::assertion_error::AssertionError;

#[macro_export]
macro_rules! match_pattern {
    ($($pattern:pat_param)|+) => {
        |received: &_| {
            match *received {
                $($pattern)|+ => Ok(()),
                _ => {
                    let received = format!("{:?}", received).red().bold();
                    Err(AssertionError { message: vec![format!("Expected {} to match pattern", received)] })
                },
            }
        }
    };
}

#[macro_export]
macro_rules! not_match_pattern {
    ($($pattern:pat_param)|+) => {
        |received: &_| {
            match *received {
                $($pattern)|+ => {
                    let received = format!("{:?}", received).red().bold();
                    Err(AssertionError { message: vec![format!("Expected {} to not match pattern", received)] })
                },
                _ => Ok(()),
            }
        }
    };
}

pub use match_pattern;
pub use not_match_pattern;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected_err::expected_err;
    use colored::control::set_override;

    #[test]
    fn test_match_pattern_ok() {
        assert_eq!(match_pattern!(1)(&1), Ok(()));
    }

    #[test]
    fn test_match_pattern_err() {
        set_override(false);
        assert_eq!(
            match_pattern!(1)(&2),
            expected_err(vec!["Expected 2 to match pattern"])
        );
    }

    #[test]
    fn test_not_match_pattern_ok() {
        assert_eq!(not_match_pattern!(1)(&2), Ok(()));
    }

    #[test]
    fn test_not_match_pattern_err() {
        set_override(false);
        assert_eq!(
            not_match_pattern!(1)(&1),
            expected_err(vec!["Expected 1 to not match pattern"])
        );
    }
}
