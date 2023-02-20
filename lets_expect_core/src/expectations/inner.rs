use std::collections::HashSet;

use proc_macro2::Ident;
use syn::parse::Parse;

use super::{
    be_err_and::BeErrAndExpectation, be_ok_and::BeOkAndExpectation,
    be_some_and::BeSomeAndExpectation, expectation_tokens::ExpectationTokens,
    expectation_type::ExpectationType, expression::ExpressionExpectation, have::HaveExpectation,
    many::ManyExpectation,
};

pub(crate) enum InnerExpectation {
    Expression(ExpressionExpectation),
    Many(ManyExpectation<InnerExpectation>),
    Have(HaveExpectation),
    BeSomeAnd(BeSomeAndExpectation),
    BeOkAndAnd(BeOkAndExpectation),
    BeErrAnd(BeErrAndExpectation),
}

impl Parse for InnerExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if HaveExpectation::peek(&input) {
            Ok(Self::Have(input.parse::<HaveExpectation>()?))
        } else if ManyExpectation::<Self>::peek(&input) {
            Ok(Self::Many(input.parse::<ManyExpectation<Self>>()?))
        } else if BeSomeAndExpectation::peek(&input) {
            Ok(Self::BeSomeAnd(input.parse::<BeSomeAndExpectation>()?))
        } else if BeOkAndExpectation::peek(&input) {
            Ok(Self::BeOkAndAnd(input.parse::<BeOkAndExpectation>()?))
        } else if BeErrAndExpectation::peek(&input) {
            Ok(Self::BeErrAnd(input.parse::<BeErrAndExpectation>()?))
        } else {
            Ok(Self::Expression(input.parse::<ExpressionExpectation>()?))
        }
    }
}

impl ExpectationType for InnerExpectation {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Expression(expectation) => expectation.span(),
            Self::Many(expectation) => expectation.span(),
            Self::Have(expectation) => expectation.span(),
            Self::BeSomeAnd(expectation) => expectation.span(),
            Self::BeOkAndAnd(expectation) => expectation.span(),
            Self::BeErrAnd(expectation) => expectation.span(),
        }
    }

    fn identifier_string(&self) -> &str {
        match self {
            Self::Expression(expectation) => expectation.identifier_string(),
            Self::Many(expectation) => expectation.identifier_string(),
            Self::Have(expectation) => expectation.identifier_string(),
            Self::BeSomeAnd(expectation) => expectation.identifier_string(),
            Self::BeOkAndAnd(expectation) => expectation.identifier_string(),
            Self::BeErrAnd(expectation) => expectation.identifier_string(),
        }
    }

    fn tokens(
        &self,
        ident_prefix: &str,
        subject_reference: bool,
        subject_mutable: bool,
    ) -> ExpectationTokens {
        match self {
            Self::Expression(expectation) => {
                expectation.tokens(ident_prefix, subject_reference, subject_mutable)
            }
            Self::Many(expectation) => {
                expectation.tokens(ident_prefix, subject_reference, subject_mutable)
            }
            Self::Have(expectation) => expectation.tokens(ident_prefix),
            Self::BeSomeAnd(expectation) => expectation.tokens(ident_prefix),
            Self::BeOkAndAnd(expectation) => expectation.tokens(ident_prefix),
            Self::BeErrAnd(expectation) => expectation.tokens(ident_prefix),
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Expression(expectation) => expectation.dependencies(),
            Self::Many(expectation) => expectation.dependencies(),
            Self::Have(expectation) => expectation.dependencies(),
            Self::BeSomeAnd(expectation) => expectation.dependencies(),
            Self::BeOkAndAnd(expectation) => expectation.dependencies(),
            Self::BeErrAnd(expectation) => expectation.dependencies(),
        }
    }
}
