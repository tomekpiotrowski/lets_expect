use colored::Colorize;
use super::executed_assertion::ExecutedAssertion;

#[derive(Debug)]
pub struct ExecutedExpectation {
    pub call: Option<(String, String)>,
    assertions: Vec<ExecutedAssertion>,
}

impl ExecutedExpectation {
    pub fn new(call: Option<(String, String)>, assertions: Vec<ExecutedAssertion>) -> Self {
        Self { call, assertions }
    }

    pub fn failed(&self) -> bool {
        self.assertions.iter().any(|assertion| assertion.failed())
    }

    pub fn pretty_print(&self) -> String {
        let assertions = self.assertions.iter().flat_map(ExecutedAssertion::pretty_print).collect::<Vec<_>>();

        if let Some(call) = self.call.as_ref() {
            format!("{} {}", call.0.cyan(), call.1.yellow().bold()) + "\n      " + &assertions.join("\n      ")
        } else {
            assertions.join("\n      ")
        }
    }
}
