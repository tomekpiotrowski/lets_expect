use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr,
};

use crate::{
    core::keyword,
    utils::{
        expr_dependencies::expr_dependencies, mutable_token::mutable_token,
        parse_expression::parse_expr_with_mutable, to_ident::expr_to_ident,
    },
};

use super::{
    expectation_tokens::AssertionTokens, expectation_type::ExpectationType, inner::InnerExpectation,
};

use crate::expectations::expectation_tokens::ExpectationTokens;

pub(crate) struct MakeExpectation {
    mutable: bool,
    expression: Expr,
    inner: Box<InnerExpectation>,
    identifier_string: String,
}

impl MakeExpectation {
    pub fn new(mutable: bool, expression: Expr, inner: Box<InnerExpectation>) -> Self {
        let call_ident = expr_to_ident(&expression);
        let mutable_string = if mutable { "mut_" } else { "" };

        let identifier_string = format!(
            "make_{}{}_{}",
            mutable_string,
            call_ident,
            inner.identifier_string()
        );

        Self {
            mutable,
            expression,
            inner,
            identifier_string,
        }
    }
    pub fn peek(input: &ParseStream) -> bool {
        input.peek(keyword::make)
    }

    pub fn span(&self) -> Span {
        self.expression.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(&self, ident_prefix: &str) -> ExpectationTokens {
        let ident = format!("{}_{}", ident_prefix, self.identifier_string());
        let inner_tokens = self.inner.tokens(ident_prefix, &ident, self.mutable);
        let before_subject = inner_tokens.before_subject;
        let after_subject = inner_tokens.after_subject;
        let assertions = AssertionTokens::Group(
            "make".to_string(),
            self.expression.to_token_stream().to_string(),
            Box::new(inner_tokens.assertions),
        );
        let expr = &self.expression;
        let mutable = mutable_token(self.mutable, &expr.span());
        let result_variable_name = Ident::new(&ident, expr.span());
        let variable_token_stream = quote_spanned! { expr.span() =>
            let #mutable #result_variable_name = #expr;
        };
        let after_subject = quote! {
            #variable_token_stream
            #after_subject
        };
        ExpectationTokens {
            before_subject,
            after_subject,
            assertions,
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expression)
            .into_iter()
            .chain(self.inner.dependencies().into_iter())
            .collect()
    }
}

impl Parse for MakeExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::make>()?;
        let (mutable, expr) = parse_expr_with_mutable(input)?;

        let inner = input.parse::<InnerExpectation>()?;

        Ok(Self::new(mutable, expr, Box::new(inner)))
    }
}
