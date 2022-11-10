use super::{context::Context, create_module::create_module, keyword, runtime::Runtime};
use crate::core::to_ident::expr_to_ident;
use proc_macro2::{Ident, TokenStream};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, Token,
};

pub struct Expect {
    context: Context,
    subject_identifier: Ident,
    mutable: bool,
    subject: Expr,
}

impl Parse for Expect {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        parenthesized!(content in input);

        let mut mutable = false;
        let mut subject_identifier = String::new();

        if content.peek(Token![mut]) {
            content.parse::<Token![mut]>()?;
            subject_identifier = "mut_".to_string();
            mutable = true;
        }

        let subject = content.parse::<Expr>()?;
        subject_identifier.push_str(&expr_to_ident(&subject));
        let subject_identifier = Ident::new(&subject_identifier, subject.span());

        let content;
        braced!(content in input);
        let context = content.parse::<Context>()?;

        Ok(Expect {
            context,
            subject_identifier,
            mutable,
            subject,
        })
    }
}

impl Expect {
    pub fn to_tokens(&self, keyword: &keyword::expect, runtime: &Runtime) -> TokenStream {
        let runtime = runtime.extend(
            Some((self.mutable, self.subject.clone())),
            &Vec::new(),
            &Vec::new(),
            &Vec::new(),
            None,
        );
        let context = self
            .context
            .to_tokens(&keyword.span(), &runtime, &Vec::new());
        let module_identifier = Ident::new(
            &format!("expect_{}", self.subject_identifier),
            self.subject_identifier.span(),
        );
        create_module(&keyword.span(), &module_identifier, &context)
    }
}
