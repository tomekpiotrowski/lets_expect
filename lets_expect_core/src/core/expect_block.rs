use super::{expect::Expect, keyword, runtime::Runtime};
use proc_macro2::TokenStream;

pub struct ExpectBlock {
    keyword: keyword::expect,
    expect: Expect,
}

impl ExpectBlock {
    pub fn new(keyword: keyword::expect, expect: Expect) -> Self {
        Self { keyword, expect }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.expect.to_tokens(&self.keyword, runtime)
    }
}
