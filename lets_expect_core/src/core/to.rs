use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use syn::{braced, Ident};

use super::expectation::Expectation;
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
            Ok(To::Multi {
                identifier: ident,
                expectations: expectations.into_iter().collect(),
            })
        } else if input.peek(Ident) {
            let expectation = input.parse::<Expectation>()?;

            Ok(To::Single(Box::new(expectation)))
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
            To::Single(expectation) => Ident::new(
                format!("{}{}", TEST_NAME_PREFIX, &expectation.identifier()).as_str(),
                expectation.span(),
            ),
            To::Multi { identifier, .. } => Ident::new(
                format!("{}{}", TEST_NAME_PREFIX, identifier).as_str(),
                identifier.span(),
            ),
        }
    }

    fn expectations(&self) -> Vec<Expectation> {
        match self {
            To::Single(expectation) => vec![(**expectation).clone()],
            To::Multi { expectations, .. } => expectations.clone(),
        }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
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
                    expectation.assertion_tokens(&prefix),
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

        let subject_label = subject.to_token_stream().to_string();

        let subject_may_panic = self
            .expectations()
            .iter()
            .any(Expectation::subject_may_panic);

        let subject_result = if subject_may_panic {
            quote_spanned! { identifier.span() =>
                let subject_result = std::panic::catch_unwind(|| {
                    #subject
                });
            }
        } else {
            quote_spanned! { identifier.span() =>
                let subject_result = { #subject };
            }
        };

        let expectations = expectations.iter().map(|(expectation, _, _, assertions)| {
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
        }).collect::<Vec<_>>();

        quote_spanned! { identifier.span() =>
            #(#before_subject)*

            #[allow(unused_variables)]
            #subject_result

            #(#after_subject)*

            let mut expectation_results: Vec<ExecutedExpectation> = Vec::new();
            #(#expectations)*

            ExecutedTestCase::new(#subject_label.to_string(), expectation_results)
        }
    }
}
