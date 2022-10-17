use proc_macro2::{Ident, TokenStream};
use super::{when::When, runtime::Runtime};

pub struct WhenBlock {
    keyword: Ident,
    when: When
}

impl WhenBlock {
    pub fn new(keyword: Ident, when: When) -> Self {
        Self { keyword, when }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.when.to_tokens(&self.keyword, runtime)
    }
}