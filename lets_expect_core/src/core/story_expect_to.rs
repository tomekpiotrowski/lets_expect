use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Expr, parse::Parse, Ident};

use crate::core::to::To;

use super::{to_block::ToBlock, runtime::Runtime};

pub struct StoryExpectTo {
    keyword: Ident,
    subject: Expr,
    to: ToBlock
}

impl StoryExpectTo {
    pub fn new(keyword: Ident, subject: Expr, to: ToBlock) -> Self {
        StoryExpectTo { keyword, subject, to }
    }
}

impl Parse for StoryExpectTo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword = input.parse::<Ident>()?;

        if keyword != "expect" {
            return Err(syn::Error::new(keyword.span(), "Expected 'expect' keyword"));
        }

        let content;
        syn::parenthesized!(content in input);

        let subject = content.parse::<Expr>()?;

        let to_keyword = input.parse::<Ident>()?;

        if to_keyword != "to" {
            return Err(syn::Error::new(to_keyword.span(), "Expected 'to' keyword"));
        }

        let to = input.parse::<To>()?;
        let to = ToBlock::new(to_keyword, to);

        Ok(StoryExpectTo::new(keyword, subject, to))
    }
}

impl StoryExpectTo {
    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        let runtime = runtime.extend(Some(self.subject.clone()), &[]);
        let to_tokens = self.to.to_tokens(&runtime);

        quote_spanned! { self.keyword.span() =>
            let test_case = { #to_tokens };
            let failed = test_case.failed();

            test_cases.push(test_case);

            if failed {
                return test_result_from_cases(test_cases);
            }
        }
    }
}