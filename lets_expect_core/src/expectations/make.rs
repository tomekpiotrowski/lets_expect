use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::{
    core::keyword,
    utils::{
        expr_dependencies::expr_dependencies,
        mutable_token::mutable_token,
        parse_expression::{parse_expectation_expression, ExpectationExpression},
        reference_token::reference_token,
        to_ident::expr_to_ident,
    },
};

use super::{
    expectation_tokens::{AssertionTokens, GroupAssertionTokens},
    expectation_type::ExpectationType,
    inner::InnerExpectation,
};

use crate::expectations::expectation_tokens::ExpectationTokens;

pub(crate) struct MakeExpectation {
    expectation_expression: ExpectationExpression,
    inner: Box<InnerExpectation>,
    identifier_string: String,
}

impl MakeExpectation {
    pub fn new(
        expectation_expression: ExpectationExpression,
        inner: Box<InnerExpectation>,
    ) -> Self {
        let expr_ident = expr_to_ident(&expectation_expression.expr);

        let ref_string = if expectation_expression.reference {
            "ref_"
        } else {
            ""
        };

        let mutable_string = if expectation_expression.mutable {
            "mut_"
        } else {
            ""
        };

        let identifier_string = format!(
            "make_{}{}{}_{}",
            ref_string,
            mutable_string,
            expr_ident,
            inner.identifier_string()
        );

        Self {
            expectation_expression,
            inner,
            identifier_string,
        }
    }
    pub fn peek(input: &ParseStream) -> bool {
        input.peek(keyword::make)
    }

    pub fn span(&self) -> Span {
        self.expectation_expression.expr.span()
    }

    pub fn identifier_string(&self) -> &str {
        &self.identifier_string
    }

    pub(crate) fn tokens(&self, ident_prefix: &str) -> ExpectationTokens {
        let inner_tokens = self.inner.tokens(
            ident_prefix,
            self.expectation_expression.reference,
            self.expectation_expression.mutable,
        );
        let before_subject = inner_tokens.before_subject_evaluation;
        let expr = &self.expectation_expression.expr;

        let reference = reference_token(self.expectation_expression.reference, &expr.span());
        let mutable = mutable_token(self.expectation_expression.mutable, &expr.span());

        let context = quote_spanned! { expr.span() =>
            let #mutable subject = #reference #expr;
        };
        let assertions = AssertionTokens::Group(GroupAssertionTokens::new(
            "make".to_string(),
            self.expectation_expression
                .expr
                .to_token_stream()
                .to_string(),
            None,
            Some(context),
            inner_tokens.assertions,
        ));
        ExpectationTokens {
            before_subject_evaluation: before_subject,
            assertions,
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        expr_dependencies(&self.expectation_expression.expr)
            .into_iter()
            .chain(self.inner.dependencies().into_iter())
            .collect()
    }
}

impl Parse for MakeExpectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::make>()?;
        let expectation_expression = parse_expectation_expression(input)?;

        let inner = input.parse::<InnerExpectation>()?;

        Ok(Self::new(expectation_expression, Box::new(inner)))
    }
}
