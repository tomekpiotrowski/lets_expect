use proc_macro2::{TokenStream, Span};
use syn::{Ident, parse::{Parse, ParseStream, ParseBuffer}, Expr, spanned::Spanned, Error, token::{Comma, Brace}, punctuated::Punctuated};
use quote::{quote_spanned, ToTokens};
use super::to_ident::{expr_to_ident, path_to_ident};
use syn::braced;

#[derive(Clone)]
pub enum Expectation {
    Single(Expr),
    Have(Expr, Vec<Expr>),
    Make(Expr, Vec<Expr>),
    Change(Expr, Vec<Expr>),
}

impl Expectation {
    pub fn subject_may_panic(&self) -> bool {
        match self {
            Expectation::Single(expr) => parse_panic_assertion(expr),
            Expectation::Have(_, _) => false,
            Expectation::Make(_, _) => false,
            Expectation::Change(_, _) => false,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expectation::Single(expr) => expr.span(),
            Expectation::Have(expr, _) => expr.span(),
            Expectation::Make(expr, _) => expr.span(),
            Expectation::Change(expr, _) => expr.span(),
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Expectation::Single(expr) => expr_to_ident(expr),
            Expectation::Have(call, assertions) => {
                let call_ident = expr_to_ident(call);

                format!("have_{}_{}", call_ident, expr_to_ident(assertions.first().expect("Expected at least one assertion")))
            },
            Expectation::Make(call, assertions) => {
                let call_ident = expr_to_ident(call);

                format!("make_{}_{}", call_ident, expr_to_ident(assertions.first().expect("Expected at least one assertion")))
            },
            Expectation::Change(call, assertions) => {
                let call_ident = expr_to_ident(call);

                format!("change_{}_{}", call_ident, expr_to_ident(assertions.first().expect("Expected at least one assertion")))
            },
        }
    }

    pub fn before_subject_tokens(&self, expectation: &String) -> TokenStream {
        match self {
            Expectation::Single(_) => TokenStream::new(),
            Expectation::Have(_, _) => TokenStream::new(),
            Expectation::Make(_, _) => TokenStream::new(),
            Expectation::Change(call, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);
                let before_variable_name = Ident::new(format!("{}_before", expectation).as_str(), call.span());

                if call_may_panic {
                    quote_spanned! { call.span() =>
                        let #before_variable_name = panic::catch_unwind(|| { #call });
                    }
                } else {
                    quote_spanned! { call.span() =>
                        let #before_variable_name = #call;
                    }
                }
            },
        }
    }

    pub fn after_subject_tokens(&self, expectation: &String) -> TokenStream {
        match self {
            Expectation::Single(_) => TokenStream::new(),
            Expectation::Have(call, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);

                let result_variable_name = Ident::new(expectation.as_str(), call.span());

                if call_may_panic {
                    quote_spanned! { call.span() =>
                        let #result_variable_name = panic::catch_unwind(|| { subject_result.#call });
                    }
                } else {
                    quote_spanned! { call.span() =>
                        let #result_variable_name = subject_result.#call;
                    }
                }
            },
            Expectation::Make(call, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);

                let result_variable_name = Ident::new(expectation.as_str(), call.span());

                if call_may_panic {
                    quote_spanned! { call.span() =>
                        let #result_variable_name = panic::catch_unwind(|| { #call });
                    }
                } else {
                    quote_spanned! { call.span() =>
                        let #result_variable_name = #call;
                    }
                }
            },
            Expectation::Change(call, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);
                let after_variable_name = Ident::new(format!("{}_after", expectation).as_str(), call.span());

                if call_may_panic {
                    quote_spanned! { call.span() =>
                        let #after_variable_name = panic::catch_unwind(|| { #call });
                    }
                } else {
                    quote_spanned! { call.span() =>
                        let #after_variable_name = #call;
                    }
                }
            },
        }
    }

    pub fn assertion_tokens(&self, expectation: &String) -> Vec<(String, TokenStream)> {
        match self {
            Expectation::Single(assertion) => {
                let assertion_label = assertion.to_token_stream().to_string();
                let assertion_may_panic = parse_panic_assertion(assertion);

                vec![(assertion_label,
                    if assertion_may_panic {
                        quote_spanned! { assertion.span() =>
                            #assertion(&subject_result)
                        }
                    } else {
                        quote_spanned! { assertion.span() =>
                            #assertion(subject_result.clone())
                        }
                    }
                )]
            },
            Expectation::Have(_, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);

                assertions.iter().map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());

                    (assertion_label, if call_may_panic {
                        quote_spanned! { assertion.span() =>
                            #assertion(&#result_variable_name)
                        }
                    } else {
                        quote_spanned! { assertion.span() =>
                            #assertion(#result_variable_name.clone())
                        }
                    })
                }).collect()
            },
            Expectation::Make(_, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);

                assertions.iter().map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let result_variable_name = Ident::new(expectation.as_str(), assertion.span());

                    (assertion_label, if call_may_panic {
                        quote_spanned! { assertion.span() =>
                            #assertion(&#result_variable_name)
                        }
                    } else {
                        quote_spanned! { assertion.span() =>
                            #assertion(#result_variable_name.clone())
                        }
                    })
                }).collect()

            },
            Expectation::Change(call, assertions) => {
                let call_may_panic = assertions.iter().any(parse_panic_assertion);

                assertions.iter().map(|assertion| {
                    let assertion_label = assertion.to_token_stream().to_string();
                    let before_variable_name = Ident::new(format!("{}_before", expectation).as_str(), call.span());
                    let after_variable_name = Ident::new(format!("{}_after", expectation).as_str(), call.span());

                    (assertion_label, if call_may_panic {
                        quote_spanned! { assertion.span() =>
                            #assertion(&#before_variable_name, &#after_variable_name)
                        }
                    } else {
                        quote_spanned! { assertion.span() =>
                            #assertion(#before_variable_name.clone(), #after_variable_name.clone())
                        }
                    })
                }).collect()
            },
        }
    }

    pub fn label(&self) -> Option<(String, String)> {
        match self {
            Expectation::Single(_) => None,
            Expectation::Have(call, _) => ("have".to_string(), call.to_token_stream().to_string()).into(),
            Expectation::Make(call, _) => ("make".to_string(), call.to_token_stream().to_string()).into(),
            Expectation::Change(call, _) => ("change".to_string(), call.to_token_stream().to_string()).into(),
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
        let expr = input.parse::<Expr>()?;

        if let Expr::Call(call) = &expr {
            if let Expr::Path(path) = &*call.func {
                let ident = Ident::new(path_to_ident(path).as_str(), path.span());

                match ident.to_string().as_str() {
                    "have" => {
                        let arg = parse_arg(call)?;
                        let assertions = parse_assertions(input)?;
                        Ok(Expectation::Have(arg, assertions))
                    },
                    "make" => {
                        let arg = parse_arg(call)?;
                        let assertions = parse_assertions(input)?;
                        Ok(Expectation::Make(arg, assertions))
                    },
                    "change" => {
                        let arg = parse_arg(call)?;
                        let assertions = parse_assertions(input)?;
                        Ok(Expectation::Change(arg, assertions))
                    },
                    _ => {
                        Ok(Expectation::Single(expr))
                    },
                }
            } else {
                Ok(Expectation::Single(expr))
            }
        } else {
            Ok(Expectation::Single(expr))
        }
    }
}

fn parse_arg(call: &syn::ExprCall) -> Result<Expr, Error> {
    let args = call.args.clone();

    if !args.len() == 1 {
        return Err(syn::Error::new(args.span(), "Expected one argument"));
    }

    let arg = args.first().unwrap().clone();
    Ok(arg)
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