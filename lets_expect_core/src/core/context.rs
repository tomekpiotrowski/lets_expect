use super::{
    after_block::AfterBlock, before_block::BeforeBlock, create_test::create_test, expect::Expect,
    expect_block::ExpectBlock, runtime::Runtime, story::Story, story_block::StoryBlock, to::To,
    to_block::ToBlock, when::When, when_block::WhenBlock,
};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseBuffer, ParseStream},
    spanned::Spanned,
    Block, Error, Ident, Local, Stmt, Token,
};

pub struct Context {
    lets: Vec<Local>,
    tos: Vec<ToBlock>,

    befores: Vec<BeforeBlock>,
    afters: Vec<AfterBlock>,

    expects: Vec<ExpectBlock>,
    whens: Vec<WhenBlock>,
    stories: Vec<StoryBlock>,
}

impl Parse for Context {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut befores = Vec::new();
        let mut afters = Vec::new();
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
                    "before" => {
                        let before = handle_before(ident, input)?;
                        befores.push(before);
                    }
                    "after" => {
                        let after = handle_after(ident, input)?;
                        afters.push(after);
                    }
                    "expect" => {
                        let expect = handle_expect(ident, input)?;
                        expects.push(expect);
                    }
                    "to" => {
                        let to = handle_to(ident, input)?;
                        tos.push(to);
                    }
                    "when" => {
                        let when = handle_when(ident, input)?;
                        whens.push(when);
                    }
                    "story" => {
                        let story = handle_story(ident, input)?;
                        stories.push(story);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!(
                                "Unexpected token `{}`. Expected `subject`, `to`, `let` or `when`",
                                ident
                            ),
                        ))
                    }
                }
            }
            next = input.lookahead1();
        }

        Ok(Context {
            tos,
            lets,
            befores,
            afters,
            expects,
            whens,
            stories,
        })
    }
}

fn handle_before(ident: Ident, input: ParseStream) -> syn::Result<BeforeBlock> {
    let block = input.parse::<Block>()?;
    Ok(BeforeBlock::new(ident, block))
}

fn handle_after(ident: Ident, input: ParseStream) -> syn::Result<AfterBlock> {
    let block = input.parse::<Block>()?;
    Ok(AfterBlock::new(ident, block))
}

fn handle_expect(ident: Ident, input: &ParseBuffer) -> syn::Result<ExpectBlock> {
    let expect = input.parse::<Expect>()?;
    Ok(ExpectBlock::new(ident, expect))
}

fn handle_when(keyword: Ident, input: &ParseBuffer) -> syn::Result<WhenBlock> {
    let when = input.parse::<When>()?;
    Ok(WhenBlock::new(keyword, when))
}

fn handle_to(ident: Ident, input: &ParseBuffer) -> syn::Result<ToBlock> {
    let to = input.parse::<To>()?;
    Ok(ToBlock::new(ident, to))
}

fn handle_let(lets: &mut Vec<Local>, input: &ParseBuffer) -> syn::Result<()> {
    let r#let = input.parse::<Stmt>()?;

    match r#let {
        Stmt::Local(local) => {
            lets.push(local);
        }
        _ => return Err(Error::new(r#let.span(), "Expected a `let` statement")),
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
        let runtime = runtime.extend(
            None,
            &lets,
            &self
                .befores
                .iter()
                .map(|before| before.before.clone())
                .collect::<Vec<Block>>(),
            &self
                .afters
                .iter()
                .map(|before| before.after.clone())
                .collect::<Vec<Block>>(),
        );

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
