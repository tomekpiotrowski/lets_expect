use super::{keyword, runtime::Runtime, when::When};
use proc_macro2::TokenStream;

pub struct WhenBlock {
    keyword: keyword::when,
    when: When,
}

impl WhenBlock {
    pub fn new(keyword: keyword::when, when: When) -> Self {
        Self { keyword, when }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.when.to_tokens(&self.keyword, runtime)
    }
}
