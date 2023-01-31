use std::collections::HashSet;

use proc_macro2::Ident;
use syn::parse::Parse;

use super::{
    change_expression::ChangeExpressionExpectation, change_many::ChangeManyExpectation,
    expectation_tokens::ExpectationTokens, expression::ExpressionExpectation,
    many::ManyExpectation,
};

pub enum ChangeInnerExpectation {
    Expression(ChangeExpressionExpectation),
    Many(ChangeManyExpectation),
}

impl Parse for ChangeInnerExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if ManyExpectation::<ExpressionExpectation>::peek(&input) {
            Ok(Self::Many(input.parse::<ChangeManyExpectation>()?))
        } else {
            Ok(Self::Expression(
                input.parse::<ChangeExpressionExpectation>()?,
            ))
        }
    }
}

impl ChangeInnerExpectation {
    pub fn identifier_string(&self) -> &str {
        match self {
            Self::Expression(expectation) => expectation.identifier_string(),
            Self::Many(expectation) => expectation.identifier_string(),
        }
    }

    pub(crate) fn tokens(
        &self,
        ident_prefix: &str,
        before_variable: &str,
        after_variable: &str,
    ) -> ExpectationTokens {
        match self {
            Self::Expression(expectation) => {
                expectation.tokens(ident_prefix, before_variable, after_variable)
            }
            Self::Many(expectation) => {
                expectation.tokens(ident_prefix, before_variable, after_variable)
            }
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Expression(expectation) => expectation.dependencies(),
            Self::Many(expectation) => expectation.dependencies(),
        }
    }
}
