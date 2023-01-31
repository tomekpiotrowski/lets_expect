use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::{quote_spanned, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr,
};

use super::{
    change_inner::ChangeInnerExpectation,
    expectation_tokens::{AssertionTokens, ExpectationTokens},
};
use crate::{
    core::keyword,
    utils::{expr_dependencies::expr_dependencies, to_ident::expr_to_ident},
};

pub struct ChangeExpectation {
    expression: Expr,
    inner: Box<ChangeInnerExpectation>,
    identifier_string: String,
}

impl ChangeExpectation {
    pub fn new(expr: Expr, inner: Box<ChangeInnerExpectation>) -> Self {
        let identifier_string = format!(
            "change_{}_{}",
            expr_to_ident(&expr),
            inner.identifier_string()
        );

        Self {
            expression: expr,
            inner,
            identifier_string,
        }
    }

    pub fn peek(input: &ParseStream) -> bool {
        input.peek(keyword::change)
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
        let after_variable_ident = Ident::new(&after_variable_name, self.span());

        let after_subject = quote_spanned! { expr.span() =>
            let #after_variable_ident = #expr;
        };

        let inner_tokens =
            self.inner
                .tokens(ident_prefix, &before_variable_name, &after_variable_name);
        let assertions = AssertionTokens::Group(
            "change".to_string(),
            self.expression.to_token_stream().to_string(),
            Box::new(inner_tokens.assertions),
        );

        ExpectationTokens {
            before_subject,
            after_subject,
            assertions,
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        let mut dependencies = expr_dependencies(&self.expression);
        dependencies.extend(self.inner.dependencies());
        dependencies
    }
}

impl Parse for ChangeExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::change>()?;
        let content;
        parenthesized!(content in input);
        let expr = content.parse::<Expr>()?;

        let inner = input.parse::<ChangeInnerExpectation>()?;

        Ok(Self::new(expr, Box::new(inner)))
    }
}
