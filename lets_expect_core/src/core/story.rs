use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Stmt,
};

use super::expr_dependencies::stmt_dependencies;
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
        Self {
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

        Ok(Self::new(identifier, elements))
    }
}

impl Story {
    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        let (elements, dependencies): (Vec<TokenStream>, HashSet<Ident>) =
            self.elements.iter().fold(
                (Vec::new(), HashSet::new()),
                |(mut token_streams, mut dependencies), element| {
                    match element {
                        StoryElement::Statement(statement) => {
                            token_streams.push(quote! { #statement });
                            dependencies.extend(stmt_dependencies(statement));
                        }
                        StoryElement::Expect(expect) => {
                            let (token_stream, idents) = expect.to_tokens(runtime);
                            token_streams.push(token_stream);
                            dependencies.extend(idents);
                        }
                    };

                    (token_streams, dependencies)
                },
            );

        let content = quote_spanned! { self.identifier.span() =>
            let mut test_cases = Vec::new();

            #(#elements)*

            test_cases
        };

        create_test(&self.identifier, runtime, &content, &dependencies)
    }
}
