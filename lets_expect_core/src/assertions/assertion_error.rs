#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssertionError {
    pub message: Vec<String>,
}

impl AssertionError {
    pub fn new(message: Vec<String>) -> Self {
        Self { message }
    }
}
