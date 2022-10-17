use std::fmt::Display;

use colored::Colorize;

use super::executed_expectation::ExecutedExpectation;

pub struct ExecutedTestCase {
    subject: String,
    expectations: Vec<ExecutedExpectation>,
}

impl ExecutedTestCase {
    pub fn new(subject: String, expectations: Vec<ExecutedExpectation>) -> Self {
        Self { subject, expectations }
    }

    pub fn failed(&self) -> bool {
        self.expectations.iter().any(|expectation| expectation.failed())
    }
}

impl Display for ExecutedTestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expect = "Expect".cyan();
        let to = "to".cyan();
        let root = " > ".blue();
        let subject = self.subject.yellow().bold();
        let expectations = self.expectations.iter().map(ExecutedExpectation::pretty_print).collect::<Vec<String>>().join("\n    ");

        write!(f, "{}{} {} {}\n    {}\n", root, expect, subject, to, expectations)
    }
}