use std::collections::HashSet;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

use super::{
    expectation_tokens::ExpectationTokens, expectation_type::ExpectationType,
    not_panic::NotPanicExpectation, panic::PanicExpectation, return_value::ReturnValueExpectation,
};

pub(crate) enum Expectation {
    Result(ReturnValueExpectation),
    Panic(PanicExpectation),
    NotPanic(NotPanicExpectation),
}

impl Parse for Expectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if PanicExpectation::peek(&input) {
            Ok(Self::Panic(input.parse::<PanicExpectation>()?))
        } else if NotPanicExpectation::peek(&input) {
            Ok(Self::NotPanic(input.parse::<NotPanicExpectation>()?))
        } else {
            Ok(Self::Result(input.parse::<ReturnValueExpectation>()?))
        }
    }
}

impl ExpectationType for Expectation {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Panic(expectation) => expectation.span(),
            Self::NotPanic(expectation) => expectation.span(),
            Self::Result(expectation) => expectation.span(),
        }
    }

    fn identifier_string(&self) -> &str {
        match self {
            Self::Panic(expectation) => expectation.identifier_string(),
            Self::NotPanic(expectation) => expectation.identifier_string(),
            Self::Result(expectation) => expectation.identifier_string(),
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Panic(expectation) => expectation.dependencies(),
            Self::NotPanic(expectation) => expectation.dependencies(),
            Self::Result(expectation) => expectation.dependencies(),
        }
    }

    fn tokens(&self, ident_prefix: &str, subject_mutable: bool) -> ExpectationTokens {
        match self {
            Self::Panic(expectation) => expectation.tokens(),
            Self::NotPanic(expectation) => expectation.tokens(),
            Self::Result(expectation) => expectation.tokens(ident_prefix, subject_mutable),
        }
    }
}

impl Expectation {
    pub(crate) fn is_panic(&self) -> bool {
        match self {
            Self::NotPanic(_) | Self::Panic(_) => true,
            Self::Result(_) => false,
        }
    }
}
