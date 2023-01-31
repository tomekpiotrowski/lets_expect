use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::quote_spanned;
use syn::parse::{Parse, ParseStream};

use crate::core::keyword;

use super::{
    expectation_tokens::{AssertionTokens, ExpectationTokens, GroupAssertionTokens},
    expectation_type::ExpectationType,
    inner::InnerExpectation,
};

pub struct BeSomeAndExpectation {
    inner: Box<InnerExpectation>,
    identifier_string: String,
}

impl BeSomeAndExpectation {
    pub(crate) fn new(expectation: InnerExpectation) -> Self {
        let identifier_string = format!("be_some_and_{}", expectation.identifier_string());
        Self {
            inner: Box::new(expectation),
            identifier_string,
        }
    }

    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::be_some_and)
    }

    pub fn span(&self) -> Span {
        self.inner.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        self.inner.dependencies()
    }

    pub(crate) fn tokens(&self, ident_prefix: &str) -> ExpectationTokens {
        let inner_tokens = self.inner.tokens(ident_prefix, false);
        let before_subject = inner_tokens.before_subject_evaluation;

        let guard = quote_spanned! { self.span() => let Some(subject) = subject };

        let assertions = AssertionTokens::Group(GroupAssertionTokens::new(
            "be_some_and".to_string(),
            "".to_string(),
            Some(guard),
            None,
            inner_tokens.assertions,
        ));

        ExpectationTokens {
            before_subject_evaluation: before_subject,
            assertions,
        }
    }
}

impl Parse for BeSomeAndExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::be_some_and>()?;

        let inner = input.parse::<InnerExpectation>()?;

        Ok(Self::new(inner))
    }
}
