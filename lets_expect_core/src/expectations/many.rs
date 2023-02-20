use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use syn::{parse::Parse, punctuated::Punctuated, Ident};

use super::{
    expectation_tokens::{AssertionTokens, ExpectationTokens},
    expectation_type::ExpectationType,
};

pub(crate) struct ManyExpectation<Expectation: ExpectationType> {
    identifier: Option<Ident>,
    inner: Vec<Expectation>,
    content_span: Span,
    identifier_string: String,
}

impl<Expectation: ExpectationType> ManyExpectation<Expectation> {
    pub fn new(identifier: Option<Ident>, inner: Vec<Expectation>, content_span: Span) -> Self {
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
            identifier,
            inner,
            content_span,
            identifier_string,
        }
    }
    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(syn::token::Brace) || (input.peek(syn::Ident) && input.peek2(syn::token::Brace))
    }
}

impl<Expectation: ExpectationType> ExpectationType for ManyExpectation<Expectation> {
    fn span(&self) -> Span {
        self.identifier
            .as_ref()
            .map_or_else(|| self.content_span, |identifier| identifier.span())
    }

    fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    fn tokens(
        &self,
        ident_prefix: &str,
        subject_reference: bool,
        subject_mutable: bool,
    ) -> ExpectationTokens {
        let ident = format!("{}_{}", ident_prefix, self.identifier_string());
        let mut before_subject = TokenStream::new();
        let mut assertions: Vec<AssertionTokens> = Vec::new();

        self.inner.iter().for_each(|inner| {
            let inner_tokens = inner.tokens(&ident, subject_reference, subject_mutable);

            before_subject.extend(inner_tokens.before_subject_evaluation);
            assertions.push(inner_tokens.assertions);
        });

        ExpectationTokens {
            before_subject_evaluation: before_subject,
            assertions: AssertionTokens::Many(assertions),
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        self.inner
            .iter()
            .flat_map(|inner| inner.dependencies())
            .collect()
    }
}

impl<Expectation: ExpectationType> Parse for ManyExpectation<Expectation> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let identifier = if input.peek(syn::Ident) {
            Some(input.parse::<Ident>()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);
        let content_span = content.span();

        let inner = Punctuated::<Expectation, syn::Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Self::new(identifier, inner, content_span))
    }
}
