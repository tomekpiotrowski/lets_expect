use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Expr};

use crate::utils::{expr_dependencies::expr_dependencies, to_ident::expr_to_ident};

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
        before_variable_name: &str,
        after_variable_name: &str,
    ) -> ExpectationTokens {
        ExpectationTokens {
            before_subject: TokenStream::new(),
            after_subject: TokenStream::new(),
            assertions: {
                let expression = &self.expression;
                let assertion_label = expression.to_token_stream().to_string();
                let before_variable_ident = Ident::new(before_variable_name, expression.span());
                let after_variable_ident = Ident::new(after_variable_name, expression.span());

                AssertionTokens::Single((
                    assertion_label,
                    quote_spanned! { expression.span() =>
                        #expression(&#before_variable_ident, &#after_variable_ident)
                    },
                ))
            },
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expression)
    }
}
