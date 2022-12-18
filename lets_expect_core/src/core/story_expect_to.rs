use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use syn::{parse::Parse, spanned::Spanned, Expr, Token};

use crate::core::{keyword, to::To};

use super::{runtime::Runtime, to_block::ToBlock};

pub struct StoryExpectTo {
    keyword: keyword::expect,
    mutable: bool,
    subject: Expr,
    to: ToBlock,
}

impl StoryExpectTo {
    pub fn new(keyword: keyword::expect, subject: Expr, mutable: bool, to: ToBlock) -> Self {
        StoryExpectTo {
            keyword,
            subject,
            mutable,
            to,
        }
    }
}

impl Parse for StoryExpectTo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword = input.parse::<keyword::expect>()?;

        let content;
        syn::parenthesized!(content in input);

        let mut mutable = false;

        if content.peek(Token![mut]) {
            content.parse::<Token![mut]>()?;
            mutable = true;
        }

        let subject = content.parse::<Expr>()?;

        let to_keyword = input.parse::<keyword::to>()?;

        let to = input.parse::<To>()?;
        let to = ToBlock::new(to_keyword, to);

        Ok(StoryExpectTo::new(keyword, subject, mutable, to))
    }
}

impl StoryExpectTo {
    pub fn to_tokens(&self, runtime: &Runtime) -> (TokenStream, HashSet<Ident>) {
        let runtime = runtime.extend(
            Some((self.mutable, self.subject.clone())),
            &[],
            &Vec::new(),
            &Vec::new(),
            None,
        );
        let (to_tokens, dependencies) = self.to.to_tokens(&runtime);

        (
            quote_spanned! { self.keyword.span() =>
                let test_case = { #to_tokens };
                let failed = test_case.failed();

                test_cases.push(test_case);

                if failed {
                    return test_result_from_cases(test_cases);
                }
            },
            dependencies,
        )
    }
}
