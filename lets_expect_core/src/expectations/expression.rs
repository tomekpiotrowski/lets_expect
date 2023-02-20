use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Expr};

use crate::utils::{
    expr_dependencies::expr_dependencies, mutable_token::mutable_token,
    reference_token::reference_token, to_ident::expr_to_ident,
};

use super::{
    expectation_tokens::{AssertionTokens, ExpectationTokens, SingleAssertionTokens},
    expectation_type::ExpectationType,
};

pub struct ExpressionExpectation {
    expression: Expr,
    identifier_string: String,
}

impl Parse for ExpressionExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expression = input.parse::<Expr>()?;

        Ok(Self {
            identifier_string: expr_to_ident(&expression),
            expression,
        })
    }
}

impl ExpectationType for ExpressionExpectation {
    fn span(&self) -> proc_macro2::Span {
        self.expression.span()
    }

    fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    fn tokens(
        &self,
        _ident_prefix: &str,
        subject_reference: bool,
        subject_mutable: bool,
    ) -> ExpectationTokens {
        ExpectationTokens {
            before_subject_evaluation: TokenStream::new(),
            assertions: {
                let expression = &self.expression;
                let assertion_label = expression.to_token_stream().to_string();
                let reference_token = reference_token(!subject_reference, &expression.span());
                let mutable_token = mutable_token(subject_mutable, &expression.span());

                AssertionTokens::Single(SingleAssertionTokens::new(
                    assertion_label,
                    quote_spanned! { expression.span() =>
                        #expression(#reference_token #mutable_token subject)
                    },
                ))
            },
        }
    }

    fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expression)
    }
}
