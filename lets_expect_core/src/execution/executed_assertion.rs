use crate::{assertions::assertion_result::AssertionResult, utils::indent::indent};
use colored::Colorize;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq)]
pub struct ExecutedAssertion {
    pub assertion: String,
    pub result: AssertionResult,
}

impl ExecutedAssertion {
    pub fn new(assertion: String, result: AssertionResult) -> Self {
        Self { assertion, result }
    }

    pub fn failed(&self) -> bool {
        self.result.is_err()
    }

    pub fn pretty_print(&self) -> Vec<String> {
        match &self.result {
            Ok(_) => vec![format!("{} {}", "✓", self.assertion)
                .green()
                .bold()
                .to_string()],
            Err(_) => {
                let mut lines = vec![format!("{} {}", "✗", self.assertion)
                    .red()
                    .bold()
                    .to_string()];
                lines.extend(indent(&self.result.as_ref().unwrap_err().message, 1));

                lines
            }
        }
    }
}

impl Debug for ExecutedAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.assertion)
    }
}
