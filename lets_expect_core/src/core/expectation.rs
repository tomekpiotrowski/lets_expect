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
            Expectation::Single { assertion } => parse_panic_assertion(assertion),
            Expectation::Have { .. }
            | Expectation::Make { .. }
            | Expectation::Change { .. }
            | Expectation::NotChange { .. } => false,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expectation::Single { assertion } => assertion.span(),
            Expectation::Have { expr, .. } => expr.span(),
            Expectation::Make { expr, .. } => expr.span(),
            Expectation::Change { expr, .. } => expr.span(),
            Expectation::NotChange { expr, .. } => expr.span(),
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Expectation::Single { assertion, .. } => expr_to_ident(assertion),
            Expectation::Have {
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
            Expectation::Make {
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
            Expectation::Change {
                expr, assertions, ..
            } => {
                let call_ident = expr_to_ident(expr);

                format!(
                    "change_{}_{}",
                    call_ident,
                    expr_to_ident(assertions.first().expect("Expected at least one assertion"))
                )
            }
            Expectation::NotChange { expr, .. } => {
                let call_ident = expr_to_ident(expr);

                format!("not_change_{}", call_ident)
            }
        }
    }

    pub fn before_subject_tokens(&self, expectation: &String) -> TokenStream {
        match self {
            Expectation::Single { .. } => TokenStream::new(),
            Expectation::Have { .. } => TokenStream::new(),
            Expectation::Make { .. } => TokenStream::new(),
            Expectation::Change {
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
                        let #before_variable_name = &#expr;
                    }
                }
            }
            Expectation::NotChange { expr, .. } => {
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
            Expectation::Single { .. } => TokenStream::new(),
            Expectation::Have {
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
                        let #result_variable_name = &panic::catch_unwind(|| { subject.#expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #result_variable_name = &#mutable subject.#expr;
                    }
                }
            }
            Expectation::Make {
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
                        let #result_variable_name = &panic::catch_unwind(|| { #expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #result_variable_name = &#mutable #expr;
                    }
                }
            }
            Expectation::Change {
                expr, assertions, ..
            } => {
                let expr_may_panic = assertions.iter().any(parse_panic_assertion);
                let after_variable_name =
                    Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                if expr_may_panic {
                    quote_spanned! { expr.span() =>
                        let #after_variable_name = &panic::catch_unwind(|| { #expr });
                    }
                } else {
                    quote_spanned! { expr.span() =>
                        let #after_variable_name = &#expr;
                    }
                }
            }
            Expectation::NotChange { expr, .. } => {
                let after_variable_name =
                    Ident::new(format!("{}_after", expectation).as_str(), expr.span());

                quote_spanned! { expr.span() =>
                    let #after_variable_name = &#expr;
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
            Expectation::Single { assertion, .. } => {
                let assertion_label = assertion.to_token_stream().to_string();
                let mutable_token = mutable_token(subject_mutable, &assertion.span());

                vec![(
                    assertion_label,
                    quote_spanned! { assertion.span() =>
                        #assertion(&#mutable_token subject)
                    },
                )]
            }
            Expectation::Have { assertions, .. } => assertions
                .iter()
                .map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());

                    (
                        assertion_label,
                        quote_spanned! { assertion.span() =>
                            #assertion(#result_variable_name)
                        },
                    )
                })
                .collect(),
            Expectation::Make { assertions, .. } => assertions
                .iter()
                .map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());

                    (
                        assertion_label,
                        quote_spanned! { assertion.span() =>
                            #assertion(&#result_variable_name)
                        },
                    )
                })
                .collect(),
            Expectation::Change {
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
            Expectation::NotChange { expr, .. } => {
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
            Expectation::Single { .. } => None,
            Expectation::Have { expr, .. } => {
                ("have".to_string(), expr.to_token_stream().to_string()).into()
            }
            Expectation::Make { expr, .. } => {
                ("make".to_string(), expr.to_token_stream().to_string()).into()
            }
            Expectation::Change { expr, .. } => {
                ("change".to_string(), expr.to_token_stream().to_string()).into()
            }
            Expectation::NotChange { expr, .. } => {
                ("not change".to_string(), expr.to_token_stream().to_string()).into()
            }
        }
    }

    pub fn dependencies(&self) -> HashSet<Ident> {
        match self {
            Expectation::Single { assertion } => expr_dependencies(assertion),
            Expectation::Have {
                expr, assertions, ..
            } => {
                let mut dependencies = expr_dependencies(expr);
                for assertion in assertions {
                    dependencies.extend(expr_dependencies(assertion));
                }
                dependencies
            }
            Expectation::Make {
                expr, assertions, ..
            } => {
                let mut dependencies = expr_dependencies(expr);
                for assertion in assertions {
                    dependencies.extend(expr_dependencies(assertion));
                }
                dependencies
            }
            Expectation::Change {
                expr, assertions, ..
            } => {
                let mut dependencies = expr_dependencies(expr);
                for assertion in assertions {
                    dependencies.extend(expr_dependencies(assertion));
                }
                dependencies
            }
            Expectation::NotChange { expr, .. } => expr_dependencies(expr),
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

            Ok(Expectation::Have {
                keyword,
                mutable,
                expr,
                assertions,
            })
        } else if input.peek(keyword::make) {
            let keyword = input.parse::<keyword::make>()?;
            let (mutable, expr) = parse_expr_with_mutable(input)?;
            let assertions = parse_assertions(input)?;

            Ok(Expectation::Make {
                keyword,
                mutable,
                expr,
                assertions,
            })
        } else if input.peek(keyword::change) {
            let keyword = input.parse::<keyword::change>()?;
            let expr = parse_expr(input)?;
            let assertions = parse_assertions(input)?;

            Ok(Expectation::Change {
                keyword,
                expr,
                assertions,
            })
        } else if input.peek(keyword::not_change) {
            let keyword = input.parse::<keyword::not_change>()?;
            let expr = parse_expr(input)?;

            Ok(Expectation::NotChange { keyword, expr })
        } else {
            let assertion = input.parse::<Expr>()?;

            Ok(Expectation::Single { assertion })
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
