#![warn(
    clippy::use_self,
    clippy::cognitive_complexity,
    clippy::cloned_instead_of_copied,
    clippy::derive_partial_eq_without_eq,
    clippy::equatable_if_let,
    clippy::explicit_into_iter_loop,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::match_same_arms,
    clippy::needless_for_each,
    clippy::todo
)]

pub mod assertions;

pub mod bool;
pub mod change;
pub mod equality;
pub mod iterator;
pub mod match_pattern;
pub mod option;
pub mod panic;
pub mod partial_ord;
pub mod result;

#[cfg(test)]
mod expected_err;
