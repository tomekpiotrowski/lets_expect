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

pub struct BeErrAndExpectation {
    expectation: Box<InnerExpectation>,
    identifier_string: String,
}

impl BeErrAndExpectation {
    pub(crate) fn new(expectation: InnerExpectation) -> Self {
        let identifier_string = format!("be_err_and_{}", expectation.identifier_string());

        Self {
            expectation: Box::new(expectation),
            identifier_string,
        }
    }

    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::be_err_and)
    }

    pub fn span(&self) -> Span {
        self.expectation.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        self.expectation.dependencies()
    }

    pub(crate) fn tokens(&self, ident_prefix: &str) -> ExpectationTokens {
        let inner_tokens = self.expectation.tokens(ident_prefix, false, false);
        let before_subject = inner_tokens.before_subject_evaluation;

        let guard = quote_spanned! { self.span() => let Err(subject) = subject };

        let assertions = AssertionTokens::Group(GroupAssertionTokens::new(
            "be_err_and".to_string(),
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

impl Parse for BeErrAndExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::be_err_and>()?;

        let inner = input.parse::<InnerExpectation>()?;

        Ok(Self::new(inner))
    }
}
