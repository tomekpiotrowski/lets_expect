use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

pub fn mutable_token(mutable: bool, span: &Span) -> TokenStream {
    if mutable {
        quote_spanned! { *span => mut }
    } else {
        quote! {}
    }
}
