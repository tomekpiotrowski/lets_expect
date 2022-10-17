use proc_macro2::{Ident, TokenStream};
use super::{to::To, runtime::Runtime};

pub struct ToBlock {
    pub keyword: Ident,
    pub to: To
}

impl ToBlock {
    pub fn new(keyword: Ident, to: To) -> Self {
        Self { keyword, to }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.to.to_tokens(runtime)
    }

    pub fn identifier(&self) -> Ident {
        self.to.identifier()
    }
}