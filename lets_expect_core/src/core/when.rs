use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{parse::Parse, braced, parenthesized};
use crate::core::to_ident::local_to_ident;

use super::runtime::Runtime;
use super::create_module::create_module;
use super::context::Context;
use syn::Local;
use syn::Stmt;
use syn::Token;

const WHEN_IDENT_PREFIX: &str = "when_";

pub struct When {
    context: Context,
    identifier: Ident,
    lets: Vec<Local>,
}

impl Parse for When {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let mut lets: Vec<Local> = Vec::new();

        let mut next = content.lookahead1();

        while next.peek(Token![let]) {
            let local = content.parse::<Stmt>()?;

            if let Stmt::Local(local) = local {
                lets.push(local);
            } else {
                return Err(syn::Error::new(local.span(), "Expected `let`"));
            }

            next = content.lookahead1();
        }

        if lets.is_empty() {
            return Err(syn::Error::new(Span::call_site(), "Expected at least one assignment"));
        }

        let name = WHEN_IDENT_PREFIX.to_string() + lets.iter().map(local_to_ident).collect::<Vec<String>>().join("_").as_str();
        let identifier = Ident::new(name.as_str(), input.span());

        let content;
        braced!(content in input);
        let context = content.parse::<Context>()?;

        Ok(When { lets, identifier, context })
    }
}

impl When {
    pub fn to_tokens(&self, keyword: &Ident, runtime: &Runtime) -> TokenStream {
        let context = self.context.to_tokens(&keyword.span(), runtime, &self.lets);
        create_module(&keyword.span(), &self.identifier, &context)
    }
}
