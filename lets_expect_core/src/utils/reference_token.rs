use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

pub fn reference_token(reference: bool, span: &Span) -> TokenStream {
    if reference {
        quote_spanned! { *span => & }
    } else {
        quote! {}
    }
}
