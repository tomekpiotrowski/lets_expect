use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::{quote_spanned, ToTokens};
use syn::{parenthesized, spanned::Spanned, Expr};

use crate::{
    core::keyword,
    utils::{expr_dependencies::expr_dependencies, to_ident::expr_to_ident},
};

use super::expectation_tokens::{AssertionTokens, ExpectationTokens};

pub struct NotChangeExpectation {
    identifier_string: String,
    expression: Expr,
}

impl NotChangeExpectation {
    pub fn new(expr: Expr) -> Self {
        let identifier_string = format!("not_change_{}", expr_to_ident(&expr));

        Self {
            identifier_string,
            expression: expr,
        }
    }
    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        input.peek(keyword::not_change)
    }

    pub fn span(&self) -> Span {
        self.expression.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(&self, ident_prefix: &str) -> ExpectationTokens {
        let ident = format!("{}_{}", ident_prefix, self.identifier_string());

        let before_variable_name = format!("{}_before", ident);
        let before_variable_ident = Ident::new(&before_variable_name, self.span());

        let expr = &self.expression;

        let before_subject = quote_spanned! { expr.span() =>
            let #before_variable_ident = #expr;
        };

        let after_variable_name = format!("{}_after", ident);
        let after_variable_ident = Ident::new(&after_variable_name, expr.span());

        let after_subject = quote_spanned! { expr.span() =>
            let #after_variable_ident = #expr;
        };

        let assertions = AssertionTokens::Single((
            "".to_string(),
            quote_spanned! { expr.span() =>
                equal(#before_variable_ident)(&#after_variable_ident)
            },
        ));

        let assertions = AssertionTokens::Group(
            "not_change".to_string(),
            self.expression.to_token_stream().to_string(),
            Box::new(assertions),
        );

        ExpectationTokens {
            before_subject,
            after_subject,
            assertions,
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expression)
    }
}

impl syn::parse::Parse for NotChangeExpectation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::not_change>()?;
        let content;
        parenthesized!(content in input);
        let expr = content.parse::<Expr>()?;

        Ok(Self::new(expr))
    }
}
