use std::collections::HashSet;

use proc_macro2::Ident;
use syn::parse::Parse;

use super::{
    expectation_tokens::ExpectationTokens, expectation_type::ExpectationType,
    expression::ExpressionExpectation, have::HaveExpectation, many::ManyExpectation,
};

pub(crate) enum InnerExpectation {
    Expression(ExpressionExpectation),
    Many(ManyExpectation<InnerExpectation>),
    Have(HaveExpectation),
}

impl Parse for InnerExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if HaveExpectation::peek(&input) {
            Ok(Self::Have(input.parse::<HaveExpectation>()?))
        } else if ManyExpectation::<Self>::peek(&input) {
            Ok(Self::Many(input.parse::<ManyExpectation<Self>>()?))
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
        }
    }

    fn identifier_string(&self) -> &str {
        match self {
            Self::Expression(expectation) => expectation.identifier_string(),
            Self::Many(expectation) => expectation.identifier_string(),
            Self::Have(expectation) => expectation.identifier_string(),
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
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Expression(expectation) => expectation.dependencies(),
            Self::Many(expectation) => expectation.dependencies(),
            Self::Have(expectation) => expectation.dependencies(),
        }
    }
}
