use std::collections::{HashMap, HashSet};

use syn::{Ident, Local};
use topological_sort::TopologicalSort;

use super::{expr_dependencies::expr_dependencies, ident_from_pat::ident_from_pat};

struct Let {
    dependencies: HashSet<Ident>,
    statements: Vec<Local>,
}

#[derive(Debug)]
pub enum TopologicalSortError {
    CyclicDependency,
    IdentExpected,
}

pub fn topological_sort(lets: &[Local]) -> Result<Vec<Local>, TopologicalSortError> {
    let let_idents: HashSet<Ident> = lets
        .iter()
        .map(|l| ident_from_pat(&l.pat))
        .collect::<Result<HashSet<Ident>, TopologicalSortError>>()?;
    let sorted: HashMap<Ident, Let> =
        lets.iter()
            .try_fold(HashMap::new(), |mut variables, r#let| {
                add_let_statement(r#let, &mut variables, &let_idents)?;
                Ok(variables)
            })?;

    let mut ts = TopologicalSort::<&Ident>::new();
    sorted.iter().for_each(|(ident, r#let)| {
        ts.insert(ident);

        r#let.dependencies.iter().for_each(|dependency| {
            ts.add_dependency(dependency, ident);
        });
    });

    let mut result = Vec::new();

    while let Some(ident) = ts.pop() {
        let r#let = sorted
            .get(ident)
            .expect("TopologicalSort returned an unknown ident");

        result.extend(r#let.statements.iter().cloned());
    }

    if !ts.is_empty() {
        return Err(TopologicalSortError::CyclicDependency);
    }

    Ok(result)
}

fn add_let_statement(
    r#let: &Local,
    variables: &mut HashMap<Ident, Let>,
    defined_idents: &HashSet<Ident>,
) -> Result<(), TopologicalSortError> {
    let ident = ident_from_pat(&r#let.pat)?;
    let dependencies = if let Some(init) = &r#let.init {
        expr_dependencies(&init.1)
    } else {
        HashSet::new()
    };
    let dependencies: HashSet<Ident> = dependencies.intersection(defined_idents).cloned().collect();
    let depends_on_itself = dependencies.contains(&ident);
    let dependencies_without_itself = dependencies
        .into_iter()
        .filter(|dependency| *dependency != ident);

    let existing_lets = variables.get_mut(&ident);
    if let Some(existing_lets) = existing_lets {
        if !depends_on_itself {
            existing_lets.dependencies.clear();
            existing_lets.statements.clear();
        }

        existing_lets
            .dependencies
            .extend(dependencies_without_itself);
        existing_lets.statements.push(r#let.clone());
    } else {
        variables.insert(
            ident.clone(),
            Let {
                dependencies: dependencies_without_itself.collect(),
                statements: vec![r#let.clone()],
            },
        );
    }

    Ok(())
}
