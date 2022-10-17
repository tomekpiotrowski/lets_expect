//! Clean tests in Rust.
//! 
//! This crate provides a macro to write better tests in Rust. It introduces a DSL loosely inspired by [RSpec](https://rspec.info/)'s one liner syntax that allows you to write tests in a more natural, declarative way.
//! See the example below.
//! 
//! # Features
//! * clean, easy to read, TDD-friendly syntax
//! * DRY-er tests
//! * less boilerplate
//! * less code
//! * uses `cargo test`, no need to install a separate test runner
//! 
//! 
//! # Example
//!
//! ```rust
//! use lets_expect::lets_expect;
//! 
//! lets_expect! {
//!     let valid_title = "Valid title";
//!     let invalid_title = "";
//!     let valid_category = 1;
//!     let invalid_category = -1;
//! 
//!     expect(create_post(title, category_id)) {
//!         when(let title = valid_title;) {
//!             when (let category_id = valid_category;) {
//!                 to create_a_post {
//!                     be_ok,
//!                     have(unwrap().body.title) equal(valid_title),
//!                     make(posts.count) change(by(1))
//!                 }
//!             }
//! 
//!             when (let category_id = invalid_category;) {
//!                 to return_an_error {
//!                     be_err,
//!                     have(unwrap_err().message) equal("Invalid category"),
//!                     make(posts.count) not_change
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//! 
//! # Installation
//! 
//! # Guide
//! 
//! # Assertions
//! 
//! # License
//! 
//! This project is licensed under the terms of the MIT license.



pub use std::panic::*;

pub use lets_expect_macro::lets_expect;

pub use lets_expect_core::execution::test_failure::TestFailure;
pub use lets_expect_core::execution::executed_test_case::ExecutedTestCase;
pub use lets_expect_core::execution::test_result::TestResult;
pub use lets_expect_core::execution::test_result::test_result_from_cases;
pub use lets_expect_core::execution::executed_assertion::ExecutedAssertion;
pub use lets_expect_core::execution::executed_expectation::ExecutedExpectation;

pub use lets_expect_core::assertions::assertion_error::AssertionError;
pub use lets_expect_core::assertions::assertion_result::AssertionResult;

pub use lets_expect_assertions::assertions::*;
