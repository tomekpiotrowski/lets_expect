use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{parse::Parse, spanned::Spanned};

use crate::core::keyword;

use super::expectation_tokens::{AssertionTokens, ExpectationTokens};

pub(crate) struct NotPanicExpectation {
    keyword: keyword::not_panic,
    identifier_string: String,
}

impl NotPanicExpectation {
    pub fn new(keyword: keyword::not_panic) -> Self {
        Self {
            keyword,
            identifier_string: "not_panic".to_string(),
        }
    }

    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::not_panic)
    }

    pub fn span(&self) -> Span {
        self.keyword.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(&self) -> ExpectationTokens {
        let assertions = AssertionTokens::Single((
            "not_panic".to_string(),
            quote_spanned! { self.keyword.span() =>
                not_panic(&subject_result)
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

impl Parse for NotPanicExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword = input.parse::<keyword::not_panic>()?;
        Ok(Self::new(keyword))
    }
}
