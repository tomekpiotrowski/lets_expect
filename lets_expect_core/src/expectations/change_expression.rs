use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Expr};

use crate::{
    expectations::expectation_tokens::SingleAssertionTokens,
    utils::{expr_dependencies::expr_dependencies, to_ident::expr_to_ident},
};

use super::expectation_tokens::{AssertionTokens, ExpectationTokens};

pub struct ChangeExpressionExpectation {
    expression: Expr,
    identifier_string: String,
}

impl Parse for ChangeExpressionExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expression = input.parse::<Expr>()?;

        Ok(Self {
            identifier_string: expr_to_ident(&expression),
            expression,
        })
    }
}

impl ChangeExpressionExpectation {
    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(
        &self,
        _ident_prefix: &str,
        change_expression: &TokenStream,
    ) -> ExpectationTokens {
        ExpectationTokens {
            before_subject_evaluation: TokenStream::new(),
            assertions: {
                let assertion_label = self.expression.to_token_stream().to_string();
                let before_variable_ident = Ident::new("from_value", self.expression.span());

                let expression = self.expression.to_token_stream();
                AssertionTokens::Single(SingleAssertionTokens::new(
                    assertion_label,
                    quote_spanned! { change_expression.span() =>
                        #expression(&#before_variable_ident, &#change_expression)
                    },
                ))
            },
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expression)
    }
}
