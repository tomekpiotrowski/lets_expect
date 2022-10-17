use proc_macro2::{Span, TokenStream};
use syn::{parse::{Parse, ParseStream, ParseBuffer}, Ident, ext::IdentExt, Error, Local, Stmt, spanned::Spanned, Token};
use quote::quote_spanned;
use super::{to::To, when::When, expect::Expect, runtime::Runtime, when_block::WhenBlock, to_block::ToBlock, expect_block::ExpectBlock, story_block::StoryBlock, story::Story, create_test::create_test};

pub struct Context {
    lets: Vec<Local>,
    tos: Vec<ToBlock>,

    expects: Vec<ExpectBlock>,
    whens: Vec<WhenBlock>,
    stories: Vec<StoryBlock>,
}

impl Parse for Context {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tos = Vec::new();
        let mut lets = Vec::new();
        let mut whens = Vec::new();
        let mut expects = Vec::new();
        let mut stories = Vec::new();

        let mut next = input.lookahead1();

        while next.peek(Ident::peek_any) {
            if next.peek(Token![let]) {
                handle_let(&mut lets, input)?;
            } else {
                let ident = input.call(Ident::parse_any)?;

                match ident.to_string().as_str() {
                    "expect" => {
                        let expect = handle_expect(ident, input)?;
                        expects.push(expect);
                    },
                    "to" => {
                        let to = handle_to(ident, input)?;
                        tos.push(to);
                    },
                    "when" => {
                        let when = handle_when(ident, input)?;
                        whens.push(when);
                    },
                    "story" => {
                        let story = handle_story(ident, input)?;
                        stories.push(story);
                    }
                    _ => return Err(syn::Error::new(ident.span(), format!("Unexpected token `{}`. Expected `subject`, `to`, `let` or `when`", ident)))
                }
            }
            next = input.lookahead1();
        }

        Ok(Context { tos, lets, expects, whens, stories })

    }
}

fn handle_expect(ident: Ident, input: &ParseBuffer) -> Result<ExpectBlock, Error> {
    let expect = input.parse::<Expect>()?;
    Ok(ExpectBlock::new(ident, expect))
}

fn handle_when(ident: Ident, input: &ParseBuffer) -> Result<WhenBlock, Error> {
    let when = input.parse::<When>()?;
    Ok(WhenBlock::new(ident, when) )
}

fn handle_to(ident: Ident, input: &ParseBuffer) -> Result<ToBlock, Error> {
    let to = input.parse::<To>()?;
    Ok(ToBlock::new(ident, to))
}

fn handle_let(lets: &mut Vec<Local>, input: &ParseBuffer) -> Result<(), syn::Error> {
    let r#let = input.parse::<Stmt>()?;

    match r#let {
        Stmt::Local(local) => {
            lets.push(local);
        },
        _ => return Err(Error::new(r#let.span(), "Expected a `let` statement"))
    }
    Ok(())
}

fn handle_story(ident: Ident, input: &ParseBuffer) -> Result<StoryBlock, Error> {
    let story = input.parse::<Story>()?;
    Ok(StoryBlock::new(ident, story))
}

impl Context {
    pub fn to_tokens(&self, span: &Span, runtime: &Runtime, extra_lets: &[Local]) -> TokenStream {
        let mut lets = self.lets.clone();
        lets.extend(extra_lets.to_vec());
        let runtime = runtime.extend(None, &lets);

        let tos = self.tos.iter().map(|to| {
            let to_tokens = to.to_tokens(&runtime);
            let identifier = to.identifier();

            let content = quote_spanned! { identifier.span() =>
                let test_case = {
                    #to_tokens
                };

                vec![test_case]
            };

            create_test(&identifier, &runtime, &content)
        });
        let stories = self.stories.iter().map(|story| story.to_tokens(&runtime));
        let expects = self.expects.iter().map(|child| child.to_tokens(&runtime));
        let whens = self.whens.iter().map(|child| child.to_tokens(&runtime));

        quote_spanned! { *span =>
            #(#tos)*
            #(#stories)*
            #(#expects)*
            #(#whens)*
        }
    }
}
