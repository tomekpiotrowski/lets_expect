use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;

use super::{
    mode::Mode,
    runtime::Runtime,
    topological_sort::{topological_sort, TopologicalSortError},
};

pub fn create_test(identifier: &Ident, runtime: &Runtime, content: &TokenStream) -> TokenStream {
    let content = content;
    let lets = topological_sort(&runtime.lets);

    let lets = match lets {
        Ok(lets) => lets,
        Err(error) => {
            return match error {
                TopologicalSortError::CyclicDependency => {
                    quote_spanned! { identifier.span() =>
                        compile_error!("Cyclic dependency between variables detected");
                    }
                }
                TopologicalSortError::IdentExpected => {
                    quote_spanned! { identifier.span() =>
                        compile_error!("Expected an identifier in `let`");
                    }
                }
            }
        }
    };

    let befores = &runtime.befores;
    let afters = &runtime.afters;

    let test_declaration = test_declaration(identifier, runtime.mode.unwrap_or(Mode::Test));

    quote_spanned! { identifier.span() =>
        #test_declaration {
            #(#lets)*

            #(#befores)*

            let test_cases = {
                #content
            };

            #(#afters)*

            test_result_from_cases(test_cases)
        }
    }
}

fn test_declaration(identifier: &Ident, mode: Mode) -> TokenStream {
    match mode {
        Mode::Test => quote_spanned! { identifier.span() =>
            #[test]
            fn #identifier() -> Result<(), TestFailure>
        },
        Mode::PubMethod => quote_spanned! { identifier.span() =>
            pub fn #identifier() -> Result<(), TestFailure>
        },
        Mode::PubAsyncMethod => quote_spanned! { identifier.span() =>
            pub async fn #identifier() -> Result<(), TestFailure>
        },
        #[cfg(feature = "tokio")]
        Mode::TokioTest => quote_spanned! { identifier.span() =>
            #[tokio::test]
            async fn #identifier() -> Result<(), TestFailure>
        },
    }
}
