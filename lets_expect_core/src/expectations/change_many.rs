use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated, Ident};

use super::{
    change_inner::ChangeInnerExpectation,
    expectation_tokens::{AssertionTokens, ExpectationTokens},
};

pub struct ChangeManyExpectation {
    inner: Vec<ChangeInnerExpectation>,
    identifier_string: String,
}

impl ChangeManyExpectation {
    pub fn new(identifier: Option<Ident>, inner: Vec<ChangeInnerExpectation>) -> Self {
        let identifier_string = identifier.as_ref().map_or_else(
            || {
                {
                    inner
                        .iter()
                        .map(|expectation| expectation.identifier_string())
                        .collect::<Vec<&str>>()
                        .join("_and_")
                }
            },
            |ident| ident.to_string(),
        );

        Self {
            inner,
            identifier_string,
        }
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(
        &self,
        ident_prefix: &str,
        before_variable_name: &str,
        after_variable_name: &str,
    ) -> ExpectationTokens {
        let ident = format!("{}_{}", ident_prefix, self.identifier_string());
        let mut before_subject = TokenStream::new();
        let mut after_subject = TokenStream::new();
        let mut assertions = Vec::new();

        self.inner.iter().for_each(|inner| {
            let inner_tokens = inner.tokens(&ident, before_variable_name, after_variable_name);

            before_subject.extend(inner_tokens.before_subject);
            after_subject.extend(inner_tokens.after_subject);
            assertions.push(inner_tokens.assertions);
        });

        ExpectationTokens {
            before_subject,
            after_subject,
            assertions: AssertionTokens::Many(assertions),
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        self.inner
            .iter()
            .flat_map(|inner| inner.dependencies())
            .collect()
    }
}

impl Parse for ChangeManyExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let identifier = if input.peek(syn::Ident) {
            Some(input.parse::<Ident>()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);

        let inner =
            Punctuated::<ChangeInnerExpectation, syn::Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect();

        Ok(Self::new(identifier, inner))
    }
}
