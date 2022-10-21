use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;

pub fn create_module(span: &Span, identifier: &Ident, content: &TokenStream) -> TokenStream {
    let content = content;

    quote_spanned! { *span =>
        pub mod #identifier {
            #[allow(unused_imports)]
            pub use super::*;

            #content
        }
    }
}
