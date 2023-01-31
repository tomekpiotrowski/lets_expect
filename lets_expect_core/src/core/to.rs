use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

use syn::spanned::Spanned;

use syn::Ident;

use crate::expectations::expectation::Expectation;
use crate::expectations::expectation_tokens::AssertionTokens;
use crate::utils::expr_dependencies::expr_dependencies;
use crate::utils::mutable_token::mutable_token;

use super::runtime::Runtime;
use crate::expectations::expectation_type::ExpectationType;
use quote::{quote, quote_spanned, ToTokens};

const TEST_NAME_PREFIX: &str = "to_";

pub struct To {
    expectation: Expectation,
}

impl Parse for To {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expectation: input.parse::<Expectation>()?,
        })
    }
}

impl To {
    pub fn identifier(&self) -> Ident {
        Ident::new(
            format!(
                "{}{}",
                TEST_NAME_PREFIX,
                &self.expectation.identifier_string()
            )
            .as_str(),
            self.expectation.span(),
        )
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> (TokenStream, HashSet<Ident>) {
        let identifier = self.identifier();
        let subject = runtime.subject.as_ref().expect("No subject set");

        let expectation_tokens = self.expectation.tokens("", "subject_result", subject.0);
        let before_subject = expectation_tokens.before_subject;
        let after_subject = expectation_tokens.after_subject;

        let subject_label = subject.1.to_token_stream().to_string();
        let subject_tokens = self.subject_tokens(subject, &identifier, self.expectation.is_panic());
        let subject_dependencies = expr_dependencies(&subject.1);

        let expectation_dependencies = self.expectation.dependencies();

        let expectation = assertion_tokens(&expectation_tokens.assertions);

        let dependencies = subject_dependencies
            .union(&expectation_dependencies)
            .cloned()
            .collect::<HashSet<_>>();

        let whens = &runtime.whens;

        let token_stream = quote_spanned! { identifier.span() =>
            #before_subject

            #[allow(unused_variables)]
            #subject_tokens

            #after_subject

            let expectation_result = #expectation;

            ExecutedTestCase::new(#subject_label.to_string(), vec![#(#whens),*], expectation_result)
        };

        (token_stream, dependencies)
    }

    fn subject_tokens(
        &self,
        subject: &(bool, syn::Expr),
        identifier: &Ident,
        is_panic: bool,
    ) -> TokenStream {
        let mutable_token = mutable_token(subject.0, &subject.1.span());
        let subject = &subject.1;

        if is_panic {
            quote_spanned! { identifier.span() =>
                #[allow(clippy::no_effect)]
                let subject_result = std::panic::catch_unwind(|| { #subject; });
            }
        } else {
            quote_spanned! { identifier.span() =>
                #[allow(clippy::let_unit_value)]
                let #mutable_token subject_result = #subject;
            }
        }
    }
}

fn assertion_tokens(tokens: &AssertionTokens) -> TokenStream {
    match tokens {
        AssertionTokens::Single(assertion) => {
            let assertion_label = &assertion.0;
            let assertion = &assertion.1;

            quote! {
                {
                    let result = #assertion;
                    ExecutedExpectation::Single(ExecutedAssertion::new(#assertion_label.to_string(), result))
                }
            }
        }
        AssertionTokens::Group(label, arg, assertion) => {
            let assertion_tokens = assertion_tokens(assertion);

            quote! {
                ExecutedExpectation::Group(#label.to_string(), #arg.to_string(), Box::new(#assertion_tokens))
            }
        }
        AssertionTokens::Many(assertions) => {
            let assertions = assertions
                .iter()
                .map(assertion_tokens)
                .collect::<Vec<TokenStream>>();

            quote! {
                ExecutedExpectation::Many(
                    vec![
                        #(#assertions),*
                    ]
                )
            }
        }
    }
}
