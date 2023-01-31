use colored::Colorize;

use crate::utils::indent::indent;

use super::executed_assertion::ExecutedAssertion;

pub enum ExecutedExpectation {
    Single(ExecutedAssertion),
    Group(String, String, Box<ExecutedExpectation>),
    Many(Vec<ExecutedExpectation>),
}

impl ExecutedExpectation {
    pub fn failed(&self) -> bool {
        match self {
            Self::Single(assertion) => assertion.failed(),
            Self::Group(_, _, assertion) => assertion.failed(),
            Self::Many(assertions) => assertions.iter().any(|assertion| assertion.failed()),
        }
    }

    pub fn pretty_print(&self) -> Vec<String> {
        match self {
            Self::Single(assertion) => assertion.pretty_print(),
            Self::Group(label, arg, assertion) => {
                let assertion = assertion.pretty_print();

                if assertion.len() == 1 {
                    vec![format!(
                        "{} {} {}",
                        label.cyan(),
                        arg.yellow().bold(),
                        assertion[0]
                    )]
                } else {
                    let assertion = indent(&assertion, 1);
                    let mut result = vec![format!("{} {}", label.cyan(), arg.yellow().bold())];
                    result.extend(assertion);
                    result
                }
            }
            Self::Many(assertions) => assertions.iter().flat_map(Self::pretty_print).collect(),
        }
    }
}
