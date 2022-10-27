use syn::{Block, Expr, Local};

use super::mode::Mode;

#[derive(Debug, Default)]
pub struct Runtime {
    pub subject: Option<Expr>,
    pub lets: Vec<Local>,
    pub befores: Vec<Block>,
    pub afters: Vec<Block>,
    pub mode: Option<Mode>,
}

impl Runtime {
    pub fn extend(
        &self,
        subject: Option<Expr>,
        lets: &[Local],
        befores: &[Block],
        afters: &[Block],
        mode: Option<Mode>,
    ) -> Runtime {
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

        let new_befores = {
            let mut new_befores = self.befores.clone();
            new_befores.extend(befores.to_owned());
            new_befores
        };

        let new_afters = {
            let mut new_afters = self.afters.clone();
            new_afters.extend(afters.to_owned());
            new_afters
        };

        let new_mode = if mode.is_some() {
            mode
        } else {
            self.mode.clone()
        };

        Runtime {
            subject: new_subject,
            lets: new_lets,
            befores: new_befores,
            afters: new_afters,
            mode: new_mode,
        }
    }
}
