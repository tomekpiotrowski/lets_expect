use syn::{Expr, Local};

#[derive(Debug, Default)]
pub struct Runtime {
    pub subject: Option<Expr>,
    pub lets: Vec<Local>,
}

impl Runtime {
    pub fn extend(&self, subject: Option<Expr>, lets: &[Local]) -> Runtime {
        let new_subject = if let Some(subject) = subject {
            Some(subject)
        } else {
            self.subject.clone()
        };

        let new_lets = {
            let mut new_lets = self.lets.clone();
            new_lets.extend(lets.to_vec());
            new_lets
        };

        Runtime {
            subject: new_subject,
            lets: new_lets,
        }
    }
}
