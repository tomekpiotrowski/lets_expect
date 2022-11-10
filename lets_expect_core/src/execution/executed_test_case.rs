use std::fmt::Display;

use colored::Colorize;

use super::{executed_expectation::ExecutedExpectation, prepend::prepend};

pub struct ExecutedTestCase {
    subject: String,
    whens: Vec<String>,
    expectations: Vec<ExecutedExpectation>,
}

impl ExecutedTestCase {
    pub fn new(subject: String, whens: Vec<&str>, expectations: Vec<ExecutedExpectation>) -> Self {
        Self {
            subject,
            whens: whens.iter().map(|when| when.to_string()).collect(),
            expectations,
        }
    }

    pub fn failed(&self) -> bool {
        self.expectations
            .iter()
            .any(|expectation| expectation.failed())
    }
}

impl Display for ExecutedTestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expect = "Expect".cyan();
        let subject = self.subject.yellow().bold();
        let mut whens = self
            .whens
            .iter()
            .enumerate()
            .map(|(index, when)| {
                format!(
                    "{}{} {}",
                    " ".repeat((index + 1) * 4),
                    "When".cyan(),
                    &when.yellow().bold()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        if !whens.is_empty() {
            whens.push('\n');
        }

        let expectations = self
            .expectations
            .iter()
            .flat_map(ExecutedExpectation::pretty_print)
            .collect::<Vec<String>>();
        let expectations = prepend(
            &expectations,
            " ".repeat((self.whens.len() + 1) * 4).as_str(),
        );

        write!(
            f,
            "{} {}\n{}{}\n",
            expect,
            subject,
            whens,
            expectations.join("\n")
        )
    }
}
