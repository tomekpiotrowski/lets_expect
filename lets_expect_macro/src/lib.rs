#![warn(
    clippy::use_self,
    clippy::cognitive_complexity,
    clippy::cloned_instead_of_copied,
    clippy::derive_partial_eq_without_eq,
    clippy::equatable_if_let,
    clippy::explicit_into_iter_loop,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::match_same_arms,
    clippy::needless_for_each,
    clippy::todo
)]

extern crate proc_macro;

use lets_expect_core::core::{context::Context, runtime::Runtime};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn lets_expect(input: TokenStream) -> TokenStream {
    lets_expect_macro(input)
}

fn lets_expect_macro(input: TokenStream) -> TokenStream {
    let main_context = parse_macro_input!(input as Context);
    let tests = main_context.to_tokens(&Span::call_site(), &Runtime::default());

    quote! {
        use lets_expect::*;

        #tests
    }
    .into()
}
