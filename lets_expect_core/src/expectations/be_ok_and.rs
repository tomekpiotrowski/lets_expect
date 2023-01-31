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

pub struct BeOkAndExpectation {
    expectation: Box<InnerExpectation>,
    identifier_string: String,
}

impl BeOkAndExpectation {
    pub(crate) fn new(expectation: InnerExpectation) -> Self {
        let identifier_string = format!("be_ok_and_{}", expectation.identifier_string());

        Self {
            expectation: Box::new(expectation),
            identifier_string,
        }
    }

    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::be_ok_and)
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
        let inner_tokens = self.expectation.tokens(ident_prefix, false);
        let before_subject = inner_tokens.before_subject_evaluation;

        let guard = quote_spanned! { self.span() => let Ok(subject) = subject };

        let assertions = AssertionTokens::Group(GroupAssertionTokens::new(
            "be_ok_and".to_string(),
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

impl Parse for BeOkAndExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::be_ok_and>()?;

        let inner = input.parse::<InnerExpectation>()?;

        Ok(Self::new(inner))
    }
}
