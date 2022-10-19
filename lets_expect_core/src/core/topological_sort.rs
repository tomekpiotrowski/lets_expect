use std::collections::{HashMap, HashSet};

use proc_macro2::TokenTree;
use syn::{Expr, Ident, Local, Pat, Stmt};
use topological_sort::TopologicalSort;

struct Let {
    dependencies: HashSet<Ident>,
    statements: Vec<Local>,
}

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
        let r#let = sorted.get(ident).unwrap();

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

fn ident_from_pat(pat: &Pat) -> Result<Ident, TopologicalSortError> {
    match pat {
        Pat::Ident(pat) => Ok(pat.ident.clone()),
        Pat::Type(pat) => Ok(ident_from_pat(&pat.pat)?),
        _ => Err(TopologicalSortError::IdentExpected),
    }
}

/// Returns the identifiers used in an expression.
fn expr_dependencies(expr: &Expr) -> HashSet<Ident> {
    let mut dependencies = HashSet::new();

    match expr {
        Expr::Binary(binary) => {
            dependencies.extend(expr_dependencies(&binary.left));
            dependencies.extend(expr_dependencies(&binary.right));
        }
        Expr::Unary(unary) => {
            dependencies.extend(expr_dependencies(&unary.expr));
        }
        Expr::Assign(assign) => {
            dependencies.extend(expr_dependencies(&assign.left));
            dependencies.extend(expr_dependencies(&assign.right));
        }
        Expr::AssignOp(assign_op) => {
            dependencies.extend(expr_dependencies(&assign_op.left));
            dependencies.extend(expr_dependencies(&assign_op.right));
        }
        Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                dependencies.insert(ident.clone());
            }
        }
        Expr::Call(call) => {
            dependencies.extend(expr_dependencies(&call.func));
            dependencies.extend(call.args.iter().flat_map(expr_dependencies));
        }
        Expr::MethodCall(method_call) => {
            dependencies.extend(expr_dependencies(&method_call.receiver));
            dependencies.extend(method_call.args.iter().flat_map(expr_dependencies));
        }
        Expr::Field(field) => {
            dependencies.extend(expr_dependencies(&field.base));
        }
        Expr::Index(index) => {
            dependencies.extend(expr_dependencies(&index.expr));
            dependencies.extend(expr_dependencies(&index.index));
        }
        Expr::Range(range) => {
            if let Some(from) = &range.from {
                dependencies.extend(expr_dependencies(from));
            }
            if let Some(to) = &range.to {
                dependencies.extend(expr_dependencies(to));
            }
        }
        Expr::Reference(reference) => {
            dependencies.extend(expr_dependencies(&reference.expr));
        }
        Expr::Paren(paren) => {
            dependencies.extend(expr_dependencies(&paren.expr));
        }
        Expr::Group(group) => {
            dependencies.extend(expr_dependencies(&group.expr));
        }
        Expr::Block(block) => {
            dependencies.extend(block.block.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::If(r#if) => {
            dependencies.extend(expr_dependencies(&r#if.cond));
            dependencies.extend(r#if.then_branch.stmts.iter().flat_map(stmt_dependencies));
            if let Some(else_branch) = &r#if.else_branch {
                dependencies.extend(expr_dependencies(&*else_branch.1));
            }
        }
        Expr::Match(match_) => {
            dependencies.extend(expr_dependencies(&match_.expr));
            dependencies.extend(
                match_
                    .arms
                    .iter()
                    .map(|arm| &*arm.body)
                    .flat_map(expr_dependencies),
            );
        }
        Expr::Closure(closure) => {
            dependencies.extend(closure.inputs.iter().flat_map(pat_dependencies));
            dependencies.extend(expr_dependencies(&closure.body));
        }
        Expr::Unsafe(unsafe_) => {
            dependencies.extend(unsafe_.block.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::Loop(r#loop) => {
            dependencies.extend(r#loop.body.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::While(while_) => {
            dependencies.extend(expr_dependencies(&while_.cond));
            dependencies.extend(while_.body.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::ForLoop(for_loop) => {
            dependencies.extend(pat_dependencies(&for_loop.pat));
            dependencies.extend(expr_dependencies(&for_loop.expr));
            dependencies.extend(for_loop.body.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::Continue(_) => {}
        Expr::Break(r#break) => {
            if let Some(expr) = &r#break.expr {
                dependencies.extend(expr_dependencies(expr));
            }
        }
        Expr::Return(return_) => {
            if let Some(expr) = &return_.expr {
                dependencies.extend(expr_dependencies(expr));
            }
        }
        Expr::Yield(yield_) => {
            if let Some(expr) = &yield_.expr {
                dependencies.extend(expr_dependencies(expr));
            }
        }
        Expr::Try(try_) => {
            dependencies.extend(expr_dependencies(&try_.expr));
        }
        Expr::Async(async_) => {
            dependencies.extend(async_.block.stmts.iter().flat_map(stmt_dependencies));
        }
        Expr::Await(r#await) => {
            dependencies.extend(expr_dependencies(&r#await.base));
        }
        Expr::Macro(macro_) => {
            dependencies.extend(
                macro_
                    .mac
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.clone()),
            );
            dependencies.extend(macro_.mac.tokens.clone().into_iter().flat_map(|token| {
                if let TokenTree::Ident(ident) = token {
                    Some(ident)
                } else {
                    None
                }
            }));
        }
        Expr::Tuple(tuple) => {
            dependencies.extend(tuple.elems.iter().flat_map(expr_dependencies));
        }
        Expr::Array(array) => {
            dependencies.extend(array.elems.iter().flat_map(expr_dependencies));
        }
        Expr::Repeat(repeat) => {
            dependencies.extend(expr_dependencies(&repeat.expr));
            dependencies.extend(expr_dependencies(&repeat.len));
        }
        Expr::Struct(r#struct) => {
            dependencies.extend(
                r#struct
                    .fields
                    .iter()
                    .flat_map(|field| expr_dependencies(&field.expr)),
            );
            dependencies.extend(
                r#struct
                    .fields
                    .iter()
                    .flat_map(|field| expr_dependencies(&field.expr)),
            );
        }
        _ => {}
    }

    dependencies
}

fn pat_dependencies(pat: &Pat) -> HashSet<Ident> {
    let mut dependencies = HashSet::new();

    match pat {
        Pat::Ident(ident) => {
            dependencies.insert(ident.ident.clone());
        }
        Pat::Type(pat_type) => {
            dependencies.extend(pat_dependencies(&pat_type.pat));
        }
        _ => {}
    }

    dependencies
}

fn stmt_dependencies(stmt: &Stmt) -> HashSet<Ident> {
    match stmt {
        Stmt::Local(local) => pat_dependencies(&local.pat),
        Stmt::Item(_) => HashSet::new(),
        Stmt::Expr(expr) => expr_dependencies(expr),
        Stmt::Semi(expr, _) => expr_dependencies(expr),
    }
}
