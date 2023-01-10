use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, Comma};
use syn::{braced, Ident};

use super::expectation::Expectation;
use super::expr_dependencies::expr_dependencies;
use super::mutable_token::mutable_token;
use super::runtime::Runtime;
use quote::{quote_spanned, ToTokens};

const TEST_NAME_PREFIX: &str = "to_";

pub enum To {
    Single(Box<Expectation>),
    Multi {
        identifier: Ident,
        expectations: Vec<Expectation>,
    },
}

impl Parse for To {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Brace) {
            let ident = input.parse::<Ident>()?;
            let content;
            braced!(content in input);

            let expectations: Punctuated<Expectation, Comma> =
                Punctuated::parse_separated_nonempty(&content)?;
            Ok(Self::Multi {
                identifier: ident,
                expectations: expectations.into_iter().collect(),
            })
        } else if input.peek(Ident) {
            let expectation = input.parse::<Expectation>()?;

            Ok(Self::Single(Box::new(expectation)))
        } else {
            Err(input.error("expected identifier or expression"))
        }
    }
}

type ExpectationTokens = (
    Expectation,
    TokenStream,
    TokenStream,
    Vec<(String, TokenStream)>,
);

impl To {
    pub fn identifier(&self) -> Ident {
        match self {
            Self::Single(expectation) => Ident::new(
                format!("{}{}", TEST_NAME_PREFIX, &expectation.identifier()).as_str(),
                expectation.span(),
            ),
            Self::Multi { identifier, .. } => Ident::new(
                format!("{}{}", TEST_NAME_PREFIX, identifier).as_str(),
                identifier.span(),
            ),
        }
    }

    fn expectations(&self) -> Vec<Expectation> {
        match self {
            Self::Single(expectation) => vec![(**expectation).clone()],
            Self::Multi { expectations, .. } => expectations.clone(),
        }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> (TokenStream, HashSet<Ident>) {
        let identifier = self.identifier();
        let subject = runtime.subject.as_ref().expect("No subject set");

        let expectations: Vec<ExpectationTokens> = self
            .expectations()
            .iter()
            .enumerate()
            .map(|(usize, expectation)| {
                let prefix = format!("expectation_{}", usize);
                (
                    expectation.clone(),
                    expectation.before_subject_tokens(&prefix),
                    expectation.after_subject_tokens(&prefix),
                    expectation.assertion_tokens(subject.0, &prefix),
                )
            })
            .collect();

        let before_subject = expectations
            .iter()
            .map(|(_, before_subject, _, _)| before_subject)
            .collect::<Vec<_>>();
        let after_subject = expectations
            .iter()
            .map(|(_, _, after_subject, _)| after_subject)
            .collect::<Vec<_>>();

        let subject_label = subject.1.to_token_stream().to_string();
        let subject_tokens = self.subject_tokens(subject, &identifier);
        let subject_dependencies = expr_dependencies(&subject.1);
        let expectation_dependencies = expectations
            .iter()
            .flat_map(|(expectation, _, _, _)| expectation.dependencies())
            .collect::<HashSet<_>>();
        let expectations = expectation_tokens(&self.identifier(), &expectations);
        let dependencies = subject_dependencies
            .union(&expectation_dependencies)
            .cloned()
            .collect::<HashSet<_>>();

        let whens = &runtime.whens;

        let token_stream = quote_spanned! { identifier.span() =>
            #(#before_subject)*

            #[allow(unused_variables)]
            #subject_tokens

            #(#after_subject)*

            let mut expectation_results: Vec<ExecutedExpectation> = Vec::new();
            #(#expectations)*

            ExecutedTestCase::new(#subject_label.to_string(), vec![#(#whens),*], expectation_results)
        };

        (token_stream, dependencies)
    }

    fn subject_tokens(&self, subject: &(bool, syn::Expr), identifier: &Ident) -> TokenStream {
        let mutable_token = mutable_token(subject.0, &subject.1.span());
        let subject_may_panic = self
            .expectations()
            .iter()
            .any(Expectation::subject_may_panic);
        let subject = &subject.1;
        if subject_may_panic {
            quote_spanned! { identifier.span() =>
                let subject = std::panic::catch_unwind(|| {
                    #subject
                });
            }
        } else {
            quote_spanned! { identifier.span() =>
                #[allow(clippy::let_unit_value)]
                let #mutable_token subject = #subject;
            }
        }
    }
}

type ExpectationComponents = (
    Expectation,
    TokenStream,
    TokenStream,
    Vec<(String, TokenStream)>,
);

fn expectation_tokens(
    identifier: &Ident,
    expectations: &[ExpectationComponents],
) -> Vec<TokenStream> {
    expectations.iter().map(|(expectation, _, _, assertions)| {
        let label = expectation.label();

        let expectation_label = if let Some(label) = label {
            let first = label.0;
            let second = label.1;

            quote_spanned! { identifier.span() =>
                Some((#first.to_string(), #second.to_string()))
            }
        } else {
            quote_spanned! { identifier.span() =>
                None
            }
        };

        let assertions = assertions.iter().map(|(label, assertion)| {
            quote_spanned!{ identifier.span() =>
                let assertion_result: AssertionResult = #assertion;
                let executed_assertion = ExecutedAssertion::new(#label.to_string(), assertion_result);
                executed_assertions.push(executed_assertion);
            }
        }).collect::<Vec<_>>();

        quote_spanned!{ identifier.span() =>
            let mut executed_assertions: Vec<ExecutedAssertion> = Vec::new();

            #(#assertions)*

            expectation_results.push(ExecutedExpectation::new(#expectation_label, executed_assertions));
        }
    }).collect::<Vec<_>>()
}
