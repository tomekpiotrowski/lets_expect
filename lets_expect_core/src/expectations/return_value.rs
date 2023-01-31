use std::collections::HashSet;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

use super::{
    change::ChangeExpectation, expectation_tokens::ExpectationTokens,
    expectation_type::ExpectationType, expression::ExpressionExpectation, have::HaveExpectation,
    make::MakeExpectation, many::ManyExpectation, not_change::NotChangeExpectation,
};

pub(crate) enum ReturnValueExpectation {
    Expression(ExpressionExpectation),
    Many(ManyExpectation<ReturnValueExpectation>),
    Have(HaveExpectation),
    Make(MakeExpectation),
    Change(ChangeExpectation),
    NotChange(NotChangeExpectation),
}

impl Parse for ReturnValueExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HaveExpectation::peek(&input) {
            Ok(Self::Have(input.parse::<HaveExpectation>()?))
        } else if MakeExpectation::peek(&input) {
            Ok(Self::Make(input.parse::<MakeExpectation>()?))
        } else if ChangeExpectation::peek(&input) {
            Ok(Self::Change(input.parse::<ChangeExpectation>()?))
        } else if NotChangeExpectation::peek(&input) {
            Ok(Self::NotChange(input.parse::<NotChangeExpectation>()?))
        } else if ManyExpectation::<Self>::peek(&input) {
            Ok(Self::Many(input.parse::<ManyExpectation<Self>>()?))
        } else {
            Ok(Self::Expression(input.parse::<ExpressionExpectation>()?))
        }
    }
}

impl ExpectationType for ReturnValueExpectation {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Expression(expectation) => expectation.span(),
            Self::Many(expectation) => expectation.span(),
            Self::Have(expectation) => expectation.span(),
            Self::Make(expectation) => expectation.span(),
            Self::Change(expectation) => expectation.span(),
            Self::NotChange(expectation) => expectation.span(),
        }
    }

    fn identifier_string(&self) -> &str {
        match self {
            Self::Expression(expectation) => expectation.identifier_string(),
            Self::Many(expectation) => expectation.identifier_string(),
            Self::Have(expectation) => expectation.identifier_string(),
            Self::Make(expectation) => expectation.identifier_string(),
            Self::Change(expectation) => expectation.identifier_string(),
            Self::NotChange(expectation) => expectation.identifier_string(),
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Expression(expectation) => expectation.dependencies(),
            Self::Many(expectation) => expectation.dependencies(),
            Self::Have(expectation) => expectation.dependencies(),
            Self::Make(expectation) => expectation.dependencies(),
            Self::Change(expectation) => expectation.dependencies(),
            Self::NotChange(expectation) => expectation.dependencies(),
        }
    }

    fn tokens(
        &self,
        ident_prefix: &str,
        subject_variable: &str,
        subject_mutable: bool,
    ) -> ExpectationTokens {
        match self {
            Self::Expression(expectation) => {
                expectation.tokens(ident_prefix, subject_variable, subject_mutable)
            }
            Self::Many(expectation) => {
                expectation.tokens(ident_prefix, subject_variable, subject_mutable)
            }
            Self::Have(expectation) => expectation.tokens(ident_prefix, subject_variable),
            Self::Make(expectation) => expectation.tokens(ident_prefix),
            Self::Change(expectation) => expectation.tokens(ident_prefix),
            Self::NotChange(expectation) => expectation.tokens(ident_prefix),
        }
    }
}
