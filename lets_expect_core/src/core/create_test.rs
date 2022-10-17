use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;

use super::runtime::Runtime;

pub fn create_test(identifier: &Ident, runtime: &Runtime, content: &TokenStream) -> TokenStream {
    let content = content;
    let lets = &runtime.lets;

    quote_spanned! { identifier.span() =>
        #[test]
        fn #identifier() -> Result<(), TestFailure> {
            #(#lets)*

            let test_cases = {
                #content
            };

            test_result_from_cases(test_cases)
        }
    }
}