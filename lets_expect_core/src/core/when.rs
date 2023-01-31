use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, Comma, Paren};
use syn::{braced, parenthesized, parse::Parse};

use crate::utils::to_ident::local_to_ident;

use super::context::Context;
use super::create_module::create_module;
use super::keyword;
use super::runtime::Runtime;
use syn::{Attribute, Expr, Local, Pat, Type};
use syn::{PatType, Token};

const WHEN_IDENT_PREFIX: &str = "when_";

struct WhenLet {
    pub attrs: Vec<Attribute>,
    pub pat: Pat,
    pub init: (Token![=], Box<Expr>),
}

impl Parse for WhenLet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut pat = input.parse()?;

        if input.peek(Token![:]) {
            let colon_token: Token![:] = input.parse()?;
            let ty: Type = input.parse()?;
            pat = Pat::Type(PatType {
                attrs: Vec::new(),
                pat: Box::new(pat),
                colon_token,
                ty: Box::new(ty),
            });
        }

        let init = (input.parse()?, input.parse()?);
        Ok(Self { attrs, pat, init })
    }
}

impl WhenLet {
    pub fn to_local(&self) -> Local {
        Local {
            attrs: self.attrs.clone(),
            let_token: Default::default(),
            pat: self.pat.clone(),
            init: Some(self.init.clone()),
            semi_token: Default::default(),
        }
    }
}

pub struct When {
    context: Context,
    identifier: Ident,
    string: String,
    lets: Vec<Local>,
}

impl Parse for When {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (lets, identifier, string) = if input.peek(Paren) {
            parse_lets_in_parentheses(input)?
        } else {
            let identifier = input.parse::<Ident>()?;
            (
                Vec::new(),
                Ident::new(
                    &format!("{}{}", WHEN_IDENT_PREFIX, identifier),
                    identifier.span(),
                ),
                identifier.to_string(),
            )
        };

        let identifier = if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            let ident = input.parse::<Ident>()?;
            Ident::new(&format!("{}{}", WHEN_IDENT_PREFIX, ident), ident.span())
        } else {
            identifier
        };

        let context = if input.peek(Brace) {
            let content;
            braced!(content in input);
            content.parse::<Context>()?
        } else {
            Context::from_single_item(input)?
        };

        Ok(Self {
            lets,
            identifier,
            string,
            context,
        })
    }
}

fn parse_lets_in_parentheses(
    input: &syn::parse::ParseBuffer,
) -> Result<(Vec<Local>, Ident, String), syn::Error> {
    let content;
    parenthesized!(content in input);

    let string = content.to_string();
    let when_lets: Punctuated<WhenLet, Comma> = Punctuated::parse_separated_nonempty(&content)?;
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
    Ok((lets, identifier, string))
}

impl When {
    pub fn to_tokens(&self, keyword: &keyword::when, runtime: &Runtime) -> TokenStream {
        let runtime = runtime.add_when(self.string.clone()).add_lets(&self.lets);
        let context = self.context.to_tokens(&keyword.span(), &runtime);
        create_module(&keyword.span(), &self.identifier, &context)
    }
}
