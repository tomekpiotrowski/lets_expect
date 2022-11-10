use super::{
    after_block::AfterBlock, before_block::BeforeBlock, create_test::create_test, expect::Expect,
    expect_block::ExpectBlock, keyword, mode::Mode, runtime::Runtime, story::Story,
    story_block::StoryBlock, to::To, to_block::ToBlock, when::When, when_block::WhenBlock,
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

    mode: Option<Mode>,
}

impl Parse for Context {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut mode = None;
        let mut befores = Vec::new();
        let mut afters = Vec::new();
        let mut tos = Vec::new();
        let mut lets = Vec::new();
        let mut whens = Vec::new();
        let mut expects = Vec::new();
        let mut stories = Vec::new();

        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            let mode_ident = input.parse::<Ident>()?;

            mode = Some(match mode_ident.to_string().as_str() {
                "test" => Mode::Test,
                "method" => Mode::PubMethod,
                "method_async" => Mode::PubAsyncMethod,
                #[cfg(feature = "tokio")]
                "tokio_test" => Mode::TokioTest,
                _ => return Err(Error::new(mode_ident.span(), "Unknown mode")),
            });
        }

        let mut next = input.lookahead1();

        while next.peek(Ident::peek_any) {
            if next.peek(Token![let]) {
                handle_let(&mut lets, input)?;
            } else if next.peek(keyword::before) {
                let keyword = input.parse::<keyword::before>()?;
                let before = handle_before(keyword, input)?;
                befores.push(before);
            } else if next.peek(keyword::after) {
                let keyword = input.parse::<keyword::after>()?;
                let after = handle_after(keyword, input)?;
                afters.push(after);
            } else if next.peek(keyword::to) {
                let keyword = input.parse::<keyword::to>()?;
                let to = handle_to(keyword, input)?;
                tos.push(to);
            } else if next.peek(keyword::when) {
                let keyword = input.parse::<keyword::when>()?;
                let when = handle_when(keyword, input)?;
                whens.push(when);
            } else if next.peek(keyword::expect) {
                let keyword = input.parse::<keyword::expect>()?;
                let expect = handle_expect(keyword, input)?;
                expects.push(expect);
            } else if next.peek(keyword::story) {
                let keyword = input.parse::<keyword::story>()?;
                let story = handle_story(keyword, input)?;
                stories.push(story);
            } else {
                return Err(next.error());
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
            mode,
        })
    }
}

fn handle_before(keyword: keyword::before, input: ParseStream) -> syn::Result<BeforeBlock> {
    let block = input.parse::<Block>()?;
    Ok(BeforeBlock::new(keyword, block))
}

fn handle_after(keyword: keyword::after, input: ParseStream) -> syn::Result<AfterBlock> {
    let block = input.parse::<Block>()?;
    Ok(AfterBlock::new(keyword, block))
}

fn handle_expect(keyword: keyword::expect, input: &ParseBuffer) -> syn::Result<ExpectBlock> {
    let expect = input.parse::<Expect>()?;
    Ok(ExpectBlock::new(keyword, expect))
}

fn handle_when(keyword: keyword::when, input: &ParseBuffer) -> syn::Result<WhenBlock> {
    let when = input.parse::<When>()?;
    Ok(WhenBlock::new(keyword, when))
}

fn handle_to(keyword: keyword::to, input: &ParseBuffer) -> syn::Result<ToBlock> {
    let to = input.parse::<To>()?;
    Ok(ToBlock::new(keyword, to))
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

fn handle_story(keyword: keyword::story, input: &ParseBuffer) -> Result<StoryBlock, Error> {
    let story = input.parse::<Story>()?;
    Ok(StoryBlock::new(keyword, story))
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
            self.mode,
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
