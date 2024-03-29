use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use syn::parse::Parse;

use super::expectation_tokens::ExpectationTokens;

pub(crate) trait ExpectationType: Parse {
    fn span(&self) -> Span;
    fn identifier_string(&self) -> &str;
    fn tokens(
        &self,
        ident_prefix: &str,
        subject_reference: bool,
        subject_mutable: bool,
    ) -> ExpectationTokens;
    fn dependencies(&self) -> HashSet<Ident>;
}
