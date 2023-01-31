use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{parse::Parse, spanned::Spanned};

use crate::core::keyword;

use super::expectation_tokens::{AssertionTokens, ExpectationTokens};

pub(crate) struct PanicExpectation {
    keyword: keyword::panic,
    identifier_string: String,
}

impl PanicExpectation {
    pub fn new(keyword: keyword::panic) -> Self {
        Self {
            keyword,
            identifier_string: "panic".to_string(),
        }
    }

    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::panic)
    }

    pub fn span(&self) -> Span {
        self.keyword.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(&self) -> ExpectationTokens {
        let assertions = AssertionTokens::Single((
            "panic".to_string(),
            quote_spanned! { self.keyword.span() =>
                panic(&subject_result)
            },
        ));

        ExpectationTokens {
            before_subject: TokenStream::new(),
            after_subject: TokenStream::new(),
            assertions,
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        HashSet::new()
    }
}

impl Parse for PanicExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword = input.parse::<keyword::panic>()?;
        Ok(Self::new(keyword))
    }
}
