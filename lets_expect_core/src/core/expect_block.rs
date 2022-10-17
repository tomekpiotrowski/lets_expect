use proc_macro2::{Ident, TokenStream};
use super::{expect::Expect, runtime::Runtime};

pub struct ExpectBlock {
    keyword: Ident,
    expect: Expect
}

impl ExpectBlock {
    pub fn new(keyword: Ident, expect: Expect) -> Self {
        Self { keyword, expect }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.expect.to_tokens(&self.keyword, runtime)
    }
}
