use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use syn::Local;

use super::{
    expr_dependencies::{block_dependencies, expr_dependencies},
    ident_from_pat::ident_from_pat,
    mode::Mode,
    runtime::Runtime,
    topological_sort::{topological_sort, TopologicalSortError},
};

pub fn create_test(
    identifier: &Ident,
    runtime: &Runtime,
    content: &TokenStream,
    dependencies: &HashSet<Ident>,
) -> TokenStream {
    let befores = &runtime.befores;
    let afters = &runtime.afters;
    let before_dependencies: HashSet<Ident> = befores.iter().flat_map(block_dependencies).collect();
    let after_dependencies: HashSet<Ident> = afters.iter().flat_map(block_dependencies).collect();

    let mut used_lets = HashSet::new();

    for dependency in dependencies
        .iter()
        .chain(before_dependencies.iter())
        .chain(after_dependencies.iter())
    {
        recursive_dependencies(&runtime.lets, dependency, &mut used_lets);
    }

    let used: Vec<Local> = runtime
        .lets
        .iter()
        .cloned()
        .filter(|l| {
            let ident = ident_from_pat(&l.pat).unwrap();
            used_lets.contains(&ident)
        })
        .collect();

    let lets = topological_sort(&used);

    let lets = match lets {
        Ok(lets) => lets,
        Err(error) => {
            return match error {
                TopologicalSortError::CyclicDependency(idents) => {
                    let error_message = format!(
                        "Cyclic dependency between variables detected: {}",
                        idents
                            .iter()
                            .map(Ident::to_string)
                            .collect::<Vec<String>>()
                            .join(", ")
                    );

                    quote_spanned! { identifier.span() =>
                        compile_error!(#error_message);
                    }
                }
                TopologicalSortError::IdentExpected => {
                    quote_spanned! { identifier.span() =>
                        compile_error!("Expected an identifier in `let`");
                    }
                }
            }
        }
    };

    let test_declaration = test_declaration(identifier, runtime.mode.unwrap_or(Mode::Test));

    quote_spanned! { identifier.span() =>
        #test_declaration {
            #(#lets)*

            #(#befores)*

            let test_cases = {
                #content
            };

            #(#afters)*

            test_result_from_cases(test_cases)
        }
    }
}

fn recursive_dependencies(lets: &[Local], ident: &Ident, dependencies: &mut HashSet<Ident>) {
    if !dependencies.contains(ident) {
        let r#let = lets.iter().find(|l| {
            let let_ident = ident_from_pat(&l.pat).expect("Expected an identifier");
            let_ident == *ident
        });

        if let Some(r#let) = r#let {
            dependencies.insert(ident.clone());

            let let_dependencies = expr_dependencies(&r#let.init.as_ref().unwrap().1);

            for dependency in let_dependencies {
                recursive_dependencies(lets, &dependency, dependencies);
            }
        }
    }
}

fn test_declaration(identifier: &Ident, mode: Mode) -> TokenStream {
    match mode {
        Mode::Test => quote_spanned! { identifier.span() =>
            #[test]
            fn #identifier() -> Result<(), TestFailure>
        },
        Mode::PubMethod => quote_spanned! { identifier.span() =>
            pub fn #identifier() -> Result<(), TestFailure>
        },
        Mode::PubAsyncMethod => quote_spanned! { identifier.span() =>
            pub async fn #identifier() -> Result<(), TestFailure>
        },
        #[cfg(feature = "tokio")]
        Mode::TokioTest => quote_spanned! { identifier.span() =>
            #[tokio::test]
            async fn #identifier() -> Result<(), TestFailure>
        },
    }
}
