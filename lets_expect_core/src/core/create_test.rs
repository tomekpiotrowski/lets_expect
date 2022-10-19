use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;

use super::{
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

    quote_spanned! { identifier.span() =>
        #[test]
        fn #identifier() -> Result<(), TestFailure> {
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
