use proc_macro2::Ident;
use syn::Pat;

use super::topological_sort::TopologicalSortError;

pub(crate) fn ident_from_pat(pat: &Pat) -> Result<Ident, TopologicalSortError> {
    match pat {
        Pat::Ident(pat) => Ok(pat.ident.clone()),
        Pat::Type(pat) => Ok(ident_from_pat(&pat.pat)?),
        _ => Err(TopologicalSortError::IdentExpected),
    }
}
