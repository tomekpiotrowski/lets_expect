use std::collections::HashSet;

use super::{keyword, runtime::Runtime, to::To};
use proc_macro2::{Ident, TokenStream};

pub struct ToBlock {
    pub _keyword: keyword::to,
    pub to: To,
}

impl ToBlock {
    pub fn new(keyword: keyword::to, to: To) -> Self {
        Self {
            _keyword: keyword,
            to,
        }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> (TokenStream, HashSet<Ident>) {
        self.to.to_tokens(runtime)
    }

    pub fn identifier(&self) -> Ident {
        self.to.identifier()
    }
}
