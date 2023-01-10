use std::collections::HashSet;

use super::expr_dependencies::expr_dependencies;
use super::keyword;
use super::mutable_token::mutable_token;
use super::to_ident::{expr_to_ident, path_to_ident};
use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{braced, parenthesized, Token};
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Comma},
    Error, Expr, Ident,
};

#[derive(Clone)]
pub enum Expectation {
    Single {
        assertion: Expr,
    },
    Have {
        keyword: keyword::have,
        mutable: bool,
        expr: Expr,
        assertions: Vec<Expr>,
    },
    Make {
        keyword: keyword::make,
        mutable: bool,
        expr: Expr,
        assertions: Vec<Expr>,
    },
    Change {
        keyword: keyword::change,
        expr: Expr,
        assertions: Vec<Expr>,
    },
    NotChange {
        keyword: keyword::not_change,
        expr: Expr,
    },
}

impl Expectation {
    pub fn subject_may_panic(&self) -> bool {
        match self {
            Self::Single { assertion } => parse_panic_assertion(assertion),
            Self::Have { .. }
            | Self::Make { .. }
            | Self::Change { .. }
            | Self::NotChange { .. } => false,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Single { assertion } => assertion.span(),
            Self::Have { expr, .. }
            | Self::Make { expr, .. }
            | Self::Change { expr, .. }
            | Self::NotChange { expr, .. } => expr.span(),
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Self::Single { assertion, .. } => expr_to_ident(assertion),
            Self::Have {
                expr,
                assertions,
                mutable,
                ..
            } => {
                let call_ident = expr_to_ident(expr);
                let mutable = if *mutable { "mut_" } else { "" };

                format!(
                    "have_{}{}_{}",
                    mutable,
                    call_ident,
                    expr_to_ident(assertions.first().expect("Expected at least one assertion"))
                )
            }
            Self::Make {
                expr,
                assertions,
                mutable,
                ..
            } => {
                let call_ident = expr_to_ident(expr);
                let mutable = if *mutable { "mut_" } else { "" };

                format!(
                    "make_{}{}_{}",
                    mutable,
                    call_ident,
                    expr_to_ident(assertions.first().expect("Expected at least one assertion"))
                )
            }
            Self::Change {
                expr, assertions, ..
            } => {
                let call_ident = expr_to_ident(expr);

                format!(
                    "change_{}_{}",
                    call_ident,
                    expr_to_ident(assertions.first().expect("Expected at least one assertion"))
                )
            }
            Self::NotChange { expr, .. } => {
                let call_ident = expr_to_ident(expr);

                format!("not_change_{}", call_ident)
            }
        }
    }

    pub fn before_subject_tokens(&self, expectation: &String) -> TokenStream {
        match self {
            Self::Have { .. } | Self::Make { .. } | Self::Single { .. } => TokenStream::new(),
            Self::Change {
                expr, assertions, ..
            } => {
                let expr_may_panic = assertions.iter().any(parse_panic_assertion);
                let before_variable_name =
                    Ident::new(format!("{}_before", expectation).as_str(), expr.span());

                if expr_may_panic {
                    quote_spanned! { expr.span() =>
                        let #before_variable_name = panic::catch_unwind(|| { #expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #before_variable_name = #expr;
                    }
                }
            }
            Self::NotChange { expr, .. } => {
                let before_variable_name =
                    Ident::new(format!("{}_before", expectation).as_str(), expr.span());

                quote_spanned! { expr.span() =>
                    let #before_variable_name = #expr;
                }
            }
        }
    }

    pub fn after_subject_tokens(&self, expectation: &String) -> TokenStream {
        match self {
            Self::Single { .. } => TokenStream::new(),
            Self::Have {
                expr,
                assertions,
                mutable,
                ..
            } => {
                let mutable = mutable_token(*mutable, &expr.span());
                let expr_may_panic = assertions.iter().any(parse_panic_assertion);
                let result_variable_name = Ident::new(expectation.as_str(), expr.span());

                if expr_may_panic {
                    quote_spanned! { expr.span() =>
                        let #result_variable_name = panic::catch_unwind(|| { subject.#expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #mutable #result_variable_name = subject.#expr;
                    }
                }
            }
            Self::Make {
                expr,
                assertions,
                mutable,
                ..
            } => {
                let mutable = mutable_token(*mutable, &expr.span());
                let expr_may_panic = assertions.iter().any(parse_panic_assertion);
                let result_variable_name = Ident::new(expectation.as_str(), expr.span());

                if expr_may_panic {
                    quote_spanned! { expr.span() =>
                        let #result_variable_name = panic::catch_unwind(|| { #expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #mutable #result_variable_name = #expr;
                    }
                }
            }
            Self::Change {
                expr, assertions, ..
            } => {
                let expr_may_panic = assertions.iter().any(parse_panic_assertion);
                let after_variable_name =
                    Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                if expr_may_panic {
                    quote_spanned! { expr.span() =>
                        let #after_variable_name = panic::catch_unwind(|| { #expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #after_variable_name = #expr;
                    }
                }
            }
            Self::NotChange { expr, .. } => {
                let after_variable_name =
                    Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                quote_spanned! { expr.span() =>
                    let #after_variable_name = #expr;
                }
            }
        }
    }

    pub fn assertion_tokens(
        &self,
        subject_mutable: bool,
        expectation: &String,
    ) -> Vec<(String, TokenStream)> {
        match self {
            Self::Single { assertion, .. } => {
                let assertion_label = assertion.to_token_stream().to_string();
                let mutable_token = mutable_token(subject_mutable, &assertion.span());

                vec![(
                    assertion_label,
                    quote_spanned! { assertion.span() =>
                        #assertion(&#mutable_token subject)
                    },
                )]
            }
            Self::Have {
                assertions,
                mutable,
                ..
            } => assertions
                .iter()
                .map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());
                    let mutable_token = mutable_token(*mutable, &assertion.span());

                    (
                        assertion_label,
                        quote_spanned! { assertion.span() =>
                            #assertion(&#mutable_token #result_variable_name)
                        },
                    )
                })
                .collect(),
            Self::Make {
                assertions,
                mutable,
                ..
            } => assertions
                .iter()
                .map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());
                    let mutable_token = mutable_token(*mutable, &assertion.span());

                    (
                        assertion_label,
                        quote_spanned! { assertion.span() =>
                            #assertion(&#mutable_token #result_variable_name)
                        },
                    )
                })
                .collect(),
            Self::Change {
                expr, assertions, ..
            } => assertions
                .iter()
                .map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let before_variable_name =
                        Ident::new(format!("{}_before", expectation).as_str(), expr.span());
                    let after_variable_name =
                        Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                    (
                        assertion_label,
                        quote_spanned! { assertion.span() =>
                            #assertion(&#before_variable_name, &#after_variable_name)
                        },
                    )
                })
                .collect(),
            Self::NotChange { expr, .. } => {
                let before_variable_name =
                    Ident::new(format!("{}_before", expectation).as_str(), expr.span());
                let after_variable_name =
                    Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                vec![(
                    "not change".to_string(),
                    quote_spanned! { expr.span() =>
                        equal(#before_variable_name)(&#after_variable_name)
                    },
                )]
            }
        }
    }

    pub fn label(&self) -> Option<(String, String)> {
        match self {
            Self::Single { .. } => None,
            Self::Have { expr, .. } => {
                ("have".to_string(), expr.to_token_stream().to_string()).into()
            }
            Self::Make { expr, .. } => {
                ("make".to_string(), expr.to_token_stream().to_string()).into()
            }
            Self::Change { expr, .. } => {
                ("change".to_string(), expr.to_token_stream().to_string()).into()
            }
            Self::NotChange { expr, .. } => {
                ("not change".to_string(), expr.to_token_stream().to_string()).into()
            }
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Self::Single { assertion } => expr_dependencies(assertion),
            Self::Have {
                expr, assertions, ..
            }
            | Self::Make {
                expr, assertions, ..
            }
            | Self::Change {
                expr, assertions, ..
            } => {
                let mut dependencies = expr_dependencies(expr);
                for assertion in assertions {
                    dependencies.extend(expr_dependencies(assertion));
                }
                dependencies
            }
            Self::NotChange { expr, .. } => expr_dependencies(expr),
        }
    }
}

fn parse_panic_assertion(assertion: &Expr) -> bool {
    if let Expr::Path(path) = assertion {
        let ident = path_to_ident(path);
        ident.ends_with("panic")
    } else {
        false
    }
}

impl Parse for Expectation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(keyword::have) {
            let keyword = input.parse::<keyword::have>()?;
            let (mutable, expr) = parse_expr_with_mutable(input)?;

            let assertions = parse_assertions(input)?;

            Ok(Self::Have {
                keyword,
                mutable,
                expr,
                assertions,
            })
        } else if input.peek(keyword::make) {
            let keyword = input.parse::<keyword::make>()?;
            let (mutable, expr) = parse_expr_with_mutable(input)?;
            let assertions = parse_assertions(input)?;

            Ok(Self::Make {
                keyword,
                mutable,
                expr,
                assertions,
            })
        } else if input.peek(keyword::change) {
            let keyword = input.parse::<keyword::change>()?;
            let expr = parse_expr(input)?;
            let assertions = parse_assertions(input)?;

            Ok(Self::Change {
                keyword,
                expr,
                assertions,
            })
        } else if input.peek(keyword::not_change) {
            let keyword = input.parse::<keyword::not_change>()?;
            let expr = parse_expr(input)?;

            Ok(Self::NotChange { keyword, expr })
        } else {
            let assertion = input.parse::<Expr>()?;

            Ok(Self::Single { assertion })
        }
    }
}

fn parse_expr_with_mutable(input: &ParseBuffer) -> Result<(bool, Expr), Error> {
    let content;
    parenthesized!(content in input);

    let mut mutable = false;

    if content.peek(Token![mut]) {
        content.parse::<Token![mut]>()?;
        mutable = true;
    }

    let expr = content.parse::<Expr>()?;
    Ok((mutable, expr))
}

fn parse_expr(input: &ParseBuffer) -> Result<Expr, Error> {
    let content;
    parenthesized!(content in input);

    let expr = content.parse::<Expr>()?;
    Ok(expr)
}

fn parse_assertions(input: &ParseBuffer) -> Result<Vec<Expr>, Error> {
    let lookahead = input.lookahead1();

    let assertions = if lookahead.peek(Brace) {
        let content;
        braced!(content in input);
        let assertions: Punctuated<Expr, Comma> = Punctuated::parse_separated_nonempty(&content)?;

        assertions.into_iter().collect()
    } else {
        let assertion = input.parse::<Expr>()?;
        vec![assertion]
    };

    Ok(assertions)
}
