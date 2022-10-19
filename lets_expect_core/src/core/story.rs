use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Stmt,
};

use super::runtime::Runtime;
use super::{create_test::create_test, story_expect_to::StoryExpectTo};

pub enum StoryElement {
    Statement(Box<Stmt>),
    Expect(Box<StoryExpectTo>),
}

pub struct Story {
    pub identifier: Ident,
    elements: Vec<StoryElement>,
}

impl Story {
    pub fn new(identifier: Ident, elements: Vec<StoryElement>) -> Self {
        Story {
            identifier,
            elements,
        }
    }
}

impl Parse for Story {
    fn parse(input: ParseStream) -> Result<Self> {
        let identifier = input.parse::<Ident>()?;

        let content;
        syn::braced!(content in input);

        let mut elements = Vec::new();

        while !content.is_empty() {
            if content.peek(Ident) && content.cursor().ident().unwrap().0 == "expect" {
                elements.push(StoryElement::Expect(content.parse()?));
            } else {
                elements.push(StoryElement::Statement(content.parse()?));
            }
        }

        Ok(Story::new(identifier, elements))
    }
}

impl Story {
    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| match element {
                StoryElement::Statement(statement) => quote! { #statement },
                StoryElement::Expect(expect) => expect.to_tokens(runtime),
            })
            .collect();

        let content = quote_spanned! { self.identifier.span() =>
            let mut test_cases = Vec::new();

            #(#elements)*

            test_cases
        };

        create_test(&self.identifier, runtime, &content)
    }
}
