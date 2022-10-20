use crate::core::to_ident::local_to_ident;
use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::token::{Paren, Semi};
use syn::{braced, parenthesized, parse::Parse};

use super::context::Context;
use super::create_module::create_module;
use super::runtime::Runtime;
use syn::Token;
use syn::{Attribute, Expr, Local, Pat};

const WHEN_IDENT_PREFIX: &str = "when_";

struct WhenLet {
    pub attrs: Vec<Attribute>,
    pub pat: Pat,
    pub init: Option<(Token![=], Box<Expr>)>,
}

impl Parse for WhenLet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let pat = input.parse()?;
        let init = if input.peek(Token![=]) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };
        Ok(WhenLet { attrs, pat, init })
    }
}

impl WhenLet {
    pub fn to_local(&self) -> Local {
        Local {
            attrs: self.attrs.clone(),
            let_token: Default::default(),
            pat: self.pat.clone(),
            init: self.init.clone(),
            semi_token: Default::default(),
        }
    }
}

pub struct When {
    context: Context,
    identifier: Ident,
    lets: Vec<Local>,
}

impl Parse for When {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (lets, identifier) = if input.peek(Paren) {
            parse_lets_in_parentheses(input)?
        } else {
            let identifier = input.parse::<Ident>()?;
            (
                Vec::new(),
                Ident::new(
                    &format!("{}{}", WHEN_IDENT_PREFIX, identifier),
                    identifier.span(),
                ),
            )
        };

        let content;
        braced!(content in input);
        let context = content.parse::<Context>()?;

        Ok(When {
            lets,
            identifier,
            context,
        })
    }
}

fn parse_lets_in_parentheses(
    input: &syn::parse::ParseBuffer,
) -> Result<(Vec<Local>, Ident), syn::Error> {
    let content;
    parenthesized!(content in input);

    let when_lets: Punctuated<WhenLet, Semi> = Punctuated::parse_separated_nonempty(&content)?;
    let lets: Vec<Local> = when_lets.iter().map(WhenLet::to_local).collect();

    if lets.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "Expected at least one assignment",
        ));
    }

    let name = WHEN_IDENT_PREFIX.to_string()
        + lets
            .iter()
            .map(local_to_ident)
            .collect::<Vec<String>>()
            .join("_")
            .as_str();
    let identifier = Ident::new(name.as_str(), input.span());
    Ok((lets, identifier))
}

impl When {
    pub fn to_tokens(&self, keyword: &Ident, runtime: &Runtime) -> TokenStream {
        let context = self.context.to_tokens(&keyword.span(), runtime, &self.lets);
        create_module(&keyword.span(), &self.identifier, &context)
    }
}
