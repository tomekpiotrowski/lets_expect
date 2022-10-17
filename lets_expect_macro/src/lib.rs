extern crate proc_macro;

use lets_expect_core::core::{runtime::Runtime, context::Context};
use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro]
pub fn lets_expect(input: TokenStream) -> TokenStream {
    lets_expect_macro(input)
}

fn lets_expect_macro(input: TokenStream) -> TokenStream {
    let expectation = parse_macro_input!(input as Context);
    let tests = expectation.to_tokens(&Span::call_site(), &Runtime::default(), &[]);

    quote! {
        use lets_expect::*;

        #tests
    }.into()
}

