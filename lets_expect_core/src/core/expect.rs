use super::{context::Context, create_module::create_module, runtime::Runtime};
use crate::core::to_ident::expr_to_ident;
use proc_macro2::{Ident, TokenStream};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr,
};

pub struct Expect {
    context: Context,
    subject_identifier: Ident,
    subject: Expr,
}

impl Parse for Expect {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        parenthesized!(content in input);
        let subject = content.parse::<Expr>()?;
        let subject_identifier = Ident::new(&expr_to_ident(&subject), subject.span());

        let content;
        braced!(content in input);
        let context = content.parse::<Context>()?;

        Ok(Expect {
            context,
            subject_identifier,
            subject,
        })
    }
}

impl Expect {
    pub fn to_tokens(&self, keyword: &Ident, runtime: &Runtime) -> TokenStream {
        let runtime = runtime.extend(
            Some(self.subject.clone()),
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
