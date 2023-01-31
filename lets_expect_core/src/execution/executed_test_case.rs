use std::fmt::Display;

use colored::Colorize;

use crate::utils::indent::indent;

use super::executed_expectation::ExecutedExpectation;

pub struct ExecutedTestCase {
    subject: String,
    whens: Vec<String>,
    expectation: ExecutedExpectation,
}

impl ExecutedTestCase {
    pub fn new(subject: String, whens: Vec<&str>, expectation: ExecutedExpectation) -> Self {
        Self {
            subject,
            whens: whens.iter().map(|when| when.to_string()).collect(),
            expectation,
        }
    }

    pub fn failed(&self) -> bool {
        self.expectation.failed()
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

        let expectations = self.expectation.pretty_print();
        let expectations = indent(&expectations, (self.whens.len() + 1) as u8);

        write!(
            f,
            "{} {} {}\n{}{}\n",
            expect,
            subject,
            "to".cyan(),
            whens,
            expectations.join("\n")
        )
    }
}
