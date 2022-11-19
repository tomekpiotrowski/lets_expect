//! Clean tests in Rust.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a + 2) {
//!     when(a = 2) {
//!         to equal(4)
//!     }
//! }
//! # }
//! # }
//! # tests::expect_a_plus_two::when_a_is_two::to_equal_four().unwrap();
//! ```
//!
//! # Why do I need this? Isn't libtest already good enough?
//!
//! How often when you see a Rust test you think to yourself "wow, this is a really beautifully written test"? Not often, right?
//! Classic Rust tests do not provide any structure beyond the test function itself. This often results in a lot of boilerplate code, ad-hoc test structure and overall
//! poor quality.
//!
//! Tests are about verifying that a given piece of code run under certain conditions works as expected. A good testing framework embraces this way of thinking.
//! It makes it easy to structure your code in a way that reflects it. Folks in other communities have been doing this for a long time with tools like
//! [RSpec](https://relishapp.com/rspec) and [Jasmine](https://jasmine.github.io/).
//!
//! If you want beautiful, high-quality tests that are a pleasure to read and write you need something else. Using Rust's procedural macros `lets_expect` introduces
//! a syntax that let's you clearly state **what** you're testing, under what **conditions** and what is the **expected result**.
//!
//! The outcome is:
//! * easy to read, DRY, TDD-friendly tests
//! * less boilerplate, less code
//! * nicer error messages
//! * more fun
//!
//! # Non-trivial example
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # #[derive(Debug, Clone)]
//! # struct Post { title: &'static str }
//! # struct Posts { posts: Vec<Post> }
//! # impl Posts {
//! #     pub fn create_post(&mut self, title: &'static str, category_id: &'static str) -> Result<Post, ValidationError> {
//! #         if category_id == "invalid category" || title == "" { Err(ValidationError { message: "Invalid category" }) } else {
//! #             let post = Post { title };
//! #             self.posts.push(post.clone());
//! #             Ok(post)
//! #         }
//! #     }
//! #
//! #     pub fn push(&mut self, post: Post) {
//! #         self.posts.push(post);
//! #     }
//! #
//! #     pub fn clear(&mut self) {
//! #         self.posts.clear();
//! #     }
//! #
//! #     pub fn len(&self) -> usize {
//! #         self.posts.len()
//! #     }
//! # }
//! # #[derive(Debug)]
//! # struct ValidationError { message: &'static str }
//! #
//! # lets_expect! { #method
//! # let valid_title = "Valid title";
//! # let invalid_title = "";
//! # let valid_category = "valid category";
//! # let invalid_category = "invalid category";
//! #
//! expect(posts.create_post(title, category_id)) {
//! #   let mut posts = Posts { posts: vec![] };
//!     before { posts.push(Post {title: "Post 1" }) }
//!     after { posts.clear() }
//!
//!     when(title = valid_title) {
//!         when(category_id = valid_category) to create_a_post {
//!             be_ok,
//!             have(as_ref().unwrap().title) equal(valid_title),
//!             change(posts.len()) { from(1), to(2) }
//!         }
//!
//!         when(category_id = invalid_category) to return_an_error {
//!             be_err,
//!             have(as_ref().unwrap_err().message) equal("Invalid category"),
//!             not_change(posts.len())
//!         }
//!     }
//!
//!     when(title = invalid_title, category_id = valid_category) to be_err
//! }
//! # }
//! # }
//! # tests::expect_posts_create_post_title_category_id::when_title_is_valid_title::when_category_id_is_valid_category::to_create_a_post().unwrap();
//! # tests::expect_posts_create_post_title_category_id::when_title_is_valid_title::when_category_id_is_invalid_category::to_return_an_error().unwrap();
//! # tests::expect_posts_create_post_title_category_id::when_title_is_invalid_title_category_id_is_valid_category::to_be_err().unwrap();
//! ```
//!
//! Now let's compare it to a classic Rust test that does the same thing:
//!
//!
//! ```
//! # use std::panic;
//! # use std::sync::Mutex;
//! # use std::ops::DerefMut;
//! # #[derive(Debug, Clone)]
//! # struct Post { title: &'static str }
//! # struct Posts { posts: Vec<Post> }
//! # impl Posts {
//! #     pub fn create_post(&mut self, title: &'static str, category_id: &'static str) -> Result<Post, ValidationError> {
//! #         if category_id == "invalid category" || title == "" { Err(ValidationError { message: "Invalid category" }) } else {
//! #             let post = Post { title };
//! #             self.posts.push(post.clone());
//! #             Ok(post)
//! #         }
//! #     }
//! #
//! #     pub fn push(&mut self, post: Post) {
//! #         self.posts.push(post);
//! #     }
//! #
//! #     pub fn clear(&mut self) {
//! #         self.posts.clear();
//! #     }
//! #
//! #     pub fn len(&self) -> usize {
//! #         self.posts.len()
//! #     }
//! # }
//! # #[derive(Debug)]
//! # struct ValidationError { message: &'static str }
//! # const VALID_TITLE: &'static str = "Valid title";
//! # const VALID_CATEGORY: &'static str = "valid category";
//! # const INVALID_CATEGORY: &'static str = "invalid category";
//! fn run_setup<T>(test: T) -> ()
//! where T: FnOnce(&mut Posts) -> () + panic::UnwindSafe
//! {
//!     let mut posts = Posts { posts: vec![] };
//!     posts.push(Post { title: "Post 1" });
//!     let posts = Mutex::new(posts);
//!     let result = panic::catch_unwind(|| {
//!         test(posts.try_lock().unwrap().deref_mut())
//!     });
//!     
//!     posts.try_lock().unwrap().clear();
//!     assert!(result.is_ok());
//! }
//!
//! #[test]
//! fn creates_a_post() {
//!     run_setup(|posts: &mut Posts| {
//!         let before_count = posts.len();
//!         let result = posts.create_post(VALID_TITLE, VALID_CATEGORY);
//!         let after_count = posts.len();
//!         assert!(result.is_ok());
//!         assert_eq!(VALID_TITLE, result.unwrap().title);
//!         assert_eq!(after_count - before_count, 1);
//!     })
//! }
//!
//! #[test]
//! fn returns_an_error_when_category_is_invalid() {
//!     run_setup(|posts: &mut Posts| {
//!         let before_count = posts.len();
//!         let result = posts.create_post(VALID_TITLE, INVALID_CATEGORY);
//!         let after_count = posts.len();
//!         assert!(result.is_err());
//!         assert_eq!("Invalid category", result.unwrap_err().message);
//!         assert_eq!(after_count, before_count);
//!     })
//! }
//!
//! #[test]
//! fn returns_an_error_when_title_is_empty() {
//!     run_setup(|posts: &mut Posts| {
//!         let result = posts.create_post("", VALID_CATEGORY);
//!         assert!(result.is_err());
//!     })
//! }
//!
//! # // creates_a_post();
//! # // returns_an_error_when_category_is_invalid();
//! # // returns_an_error_when_title_is_empty();
//! ```
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! lets_expect = "0"
//! ```
//!
//! # Guide
//!
//! ## How does Let's Expect work?
//!
//! Under the hood `lets_expect` generates a single classic test function for each `to` block. It names those tests automatically based on what you're testing and
//! organizes those tests into modules. This means you can run those tests using `cargo test` and you can use all `cargo test` features. IDE extensions will
//! also work as expected.
//!
//! `cargo test` output might look like this:
//!
//! ```text
//! running 5 tests
//! test tests::expect_a_plus_b_plus_c::when_a_is_two::when_b_is_one_c_is_one::to_equal_4 ... ok
//! test tests::expect_a_plus_b_plus_c::when_c_is_three::expect_two_plus_c_plus_ten::to_equal_fifteen ... ok
//! test tests::expect_a_plus_b_plus_c::when_a_is_three_b_is_three_c_is_three::to_equal_nine ... ok
//! test tests::expect_a_plus_b_plus_c::when_all_numbers_are_negative::to_equal_neg_six ... ok
//! test tests::expect_array::when_array_is_one_two_three::to_equal_one_two_three ... ok
//!
//! test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
//!
//! ## Where to put my tests?
//!
//! Let's Expect tests need to be placed inside of a `lets_expect!` macro, which in turn needs to be placed inside of a `tests` module:
//!
//! ```
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use lets_expect::lets_expect;
//!
//!     lets_expect! {
//!         expect(subject) {
//!             to expectation
//!         }
//!     }
//! }
//! ```
//!
//! It might be a good idea to define a code snippet in your IDE to avoid having to type this piece of boilerplate every time.
//!
//! The examples here omit the macro for brevity.
//!
//! ## `expect` and `to`
//!
//! `expect` sets the subject of the test. It can be any Rust expression (including a block). `to` introduces expectations. It can be followed
//! by a single expectation or a block of expectations. In the latter case you must provide a name for the test, which needs to be a valid Rust identifier.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(2) {
//!     to equal(2)
//! }
//! # }
//! # }
//! # tests::expect_two::to_equal_two().unwrap();
//! ```
//!
//! If there are multiple assertions in a `to` block they need to be separated by a comma.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect({ 1 + 1 }) {
//!     to be_actually_2 {
//!         equal(2),
//!         not_equal(3)
//!     }
//! }
//! # }
//! # }
//! # tests::expect_one_plus_one::to_be_actually_2().unwrap();
//! ```
//!
//! One `to` block generates a single test. This means the subject will be executed once and then all the assertions inside that `to` block will be run.
//! If you want to generate multiple tests you can use multiple `to` blocks:
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # struct Files {
//! #     created: bool
//! # }
//! # impl Files {
//! #    pub fn create_file(&mut self) {
//! #        self.created = true;
//! #    }
//! #    pub fn try_to_remove_file(&mut self) -> bool {
//! #        let result = self.created;
//! #        self.created = false;
//! #        result
//! #    }
//! #    pub fn file_exists(&self) -> bool {
//! #        self.created
//! #    }
//! # }
//! # lets_expect! { #method
//! # let mut files = Files { created: false };
//! expect(files.create_file()) {
//!     to make(files.try_to_remove_file()) be_true
//!     to make(files.file_exists()) be_true
//! }
//! # }
//! # }
//! # tests::expect_files_create_file::to_make_files_try_to_remove_file_be_true().unwrap();
//! # tests::expect_files_create_file::to_make_files_file_exists_be_true().unwrap();
//! ```
//!
//! If your `expect` contains a single item you can omit the braces:
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a + 2) when(a = 2) {
//!     to equal(4)
//! }
//! # }
//! # }
//! # tests::expect_a_plus_two::when_a_is_two::to_equal_four().unwrap();
//! ```
//!
//!
//! ## `let`
//!
//! Inside the top level `lets_expect!` macro as well as `expect` and `when` blocks you can use `let` to define variables.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a) {
//!     let a = 2;
//!
//!     to equal(2)
//! }
//! # }
//! # }
//! # tests::expect_a::to_equal_two().unwrap();
//! ```
//!
//! Variables can be overwritten in nested blocks. New definitions can use values from outer blocks.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a) {
//!     let a = 2;
//!
//!     when a_is_4 {
//!         let a = a + 2;
//!
//!         to equal(4)
//!     }
//! }
//! # }
//! # }
//! # tests::expect_a::when_a_is_4::to_equal_four().unwrap();
//! ```
//!
//! Variables don't have to be defined in the order they're used.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(sum) {
//!     let sum = a + b;
//!     let a = 2;
//!
//!     when b_is_three {
//!         let b = 3;
//!
//!         to equal(5)
//!     }
//! }
//! # }
//! # }
//! # tests::expect_sum::when_b_is_three::to_equal_five().unwrap();
//! ```
//!
//! ## `when`
//!
//! `when` sets a value of one or more variables for a given block. This keyword is this library's secret sauce. It allows you to define values of variables
//! for multiples tests in a concise and readable way, without having to repeat it in every test.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a + b + c) {
//!     let a = 2;
//!
//!     when(c = 5) {
//!         when(b = 3) {
//!             to equal(10)
//!         }
//!
//!         when(a = 10, b = 10) {
//!             to equal(25)
//!         }
//!     }
//! }
//! # }
//! # }
//! # tests::expect_a_plus_b_plus_c::when_c_is_five::when_b_is_three::to_equal_ten().unwrap();
//! # tests::expect_a_plus_b_plus_c::when_c_is_five::when_a_is_ten_b_is_ten::to_equal_twentyfive().unwrap();
//! ```
//!
//! You can use similar syntax as in `let` to define variables. The only difference being the `let` keyword itself is ommited.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a += 1) {
//!     when(mut a: i64 = 1) {
//!         to change(a.clone()) { from(1), to(2) }
//!     }
//! }
//! # }
//! # }
//! # tests::expect_a_add_equal_one::when_a_is_one::to_change_a_clone_from_one().unwrap();
//! ```
//!
//! You can also use `when` with an identifier. This will simply create a new context with the given identifier. No new variables are defined.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # pub fn login(username: &str, password: &str) -> bool {
//! #     username == "user" && password == "pass"
//! # }
//! # lets_expect! { #method
//! expect(login(username, password)) {
//!     when credentials_are_invalid {
//!         let username = "invalid";
//!         let password = "invalid";
//!
//!         to be_false
//!     }
//! }
//! # }
//! # }
//! # tests::expect_login_username_password::when_credentials_are_invalid::to_be_false().unwrap();
//! ```
//!
//! If your `when` contains only one item the braces can be ommited:
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(a + 2) when(a = 2) to equal(4)
//! # }
//! # }
//! # tests::expect_a_plus_two::when_a_is_two::to_equal_four().unwrap();
//! ```
//!
//! ## `have`
//!
//! `have` is used to test values of attributes or return values of methods of the subject.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # struct Response { pub status: u16 }
//! # impl Response { pub fn is_ok(&self) -> bool { self.status == 200 } }
//! # lets_expect! { #method
//! let response = Response { status: 200 };
//!
//! expect(response) {
//!     to be_valid {
//!         have(status) equal(200),
//!         have(is_ok()) be_true
//!     }
//! }
//! # }
//! # }
//! # tests::expect_response::to_be_valid().unwrap();
//! ```
//!
//! Multiple assertions can be provided to `have` by wrapping them in curly braces and separating them with commas.
//!
//! ## `make`
//!
//! `make` is used to test values of arbitrary expressions.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # fn user_has_posts(user_id: i32) -> bool { true }
//! # lets_expect! { #method
//! # let mut posts = Vec::new();
//! expect(posts.push((user_id, "new post"))) {
//!     let user_id = 1;
//!
//!     to make(user_has_posts(user_id)) be_true
//! }
//! # }
//! # }
//! # tests::expect_posts_push_user_id_string::to_make_user_has_posts_user_id_be_true().unwrap();
//! ```
//!
//! Multiple assertions can be provided to `make` by wrapping them in curly braces and separating them with commas.
//!
//! ## `change`
//!
//! `change` is used to test if and how a value changes after subject is executed. The expression given as an argument to `change` is evaluated twice. Once before the subject is executed and once after.
//! The two values are then provided to the assertions specified in the `change` block.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # #[derive(Debug, Clone)]
//! # struct Post { title: &'static str }
//! # struct Posts { posts: Vec<Post> }
//! # impl Posts {
//! #     pub fn create_post(&mut self, title: &'static str, category_id: &'static str) -> Result<Post, ValidationError> {
//! #         if category_id == "invalid category" || title == "" { Err(ValidationError { message: "Invalid category" }) } else {
//! #             let post = Post { title };
//! #             self.posts.push(post.clone());
//! #             Ok(post)
//! #         }
//! #     }
//! #
//! #     pub fn push(&mut self, post: Post) {
//! #         self.posts.push(post);
//! #     }
//! #
//! #     pub fn clear(&mut self) {
//! #         self.posts.clear();
//! #     }
//! #
//! #     pub fn len(&self) -> usize {
//! #         self.posts.len()
//! #     }
//! # }
//! # #[derive(Debug)]
//! # struct ValidationError { message: &'static str }
//! #
//! # lets_expect! { #method
//! # let valid_title = "Valid title";
//! # let invalid_title = "";
//! # let valid_category = "valid category";
//! # let invalid_category = "invalid category";
//! #
//! expect(posts.create_post(title, category_id)) {
//! #   let mut posts = Posts { posts: vec![] };
//!     after { posts.clear() }
//!
//!     when(title = valid_title) {
//!         when(category_id = valid_category) {
//!             to change(posts.len()) { from(0), to(1) }
//!         }
//!
//!         when(category_id = invalid_category) {
//!             to not_change(posts.len())
//!         }
//!     }
//! }
//! # }
//! # }
//! # tests::expect_posts_create_post_title_category_id::when_title_is_valid_title::when_category_id_is_valid_category::to_change_posts_len_from_zero().unwrap();
//! # tests::expect_posts_create_post_title_category_id::when_title_is_valid_title::when_category_id_is_invalid_category::to_not_change_posts_len().unwrap();
//! ```
//!
//! ## `match_pattern!`
//!
//! `match_pattern!` is used to test if a value matches a pattern. It's functionality is similar to [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro.
//!
//! ```
//! # mod tests {
//! # #[derive(Clone, Debug, PartialEq)]
//! # pub enum Response {
//! #     UserCreated,
//! #     ValidationFailed(&'static str),
//! # }
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(Response::UserCreated) {
//!     to match_pattern!(Response::UserCreated)
//! }
//!
//! expect(Response::ValidationFailed("email")) {
//!     to match_email {
//!         match_pattern!(Response::ValidationFailed("email")),
//!         not_match_pattern!(Response::ValidationFailed("email2"))
//!     }
//! }
//! # }
//! # }
//! # tests::expect_response_usercreated::to_match_pattern().unwrap();
//! # tests::expect_response_validationfailed_string::to_match_email().unwrap();
//! ```
//!
//! ## `Option` and `Result`
//!
//! Let's Expect provides a set of assertions for `Option` and `Result` types.
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # lets_expect! { #method
//! expect(Some(1u8) as Option<u8>) {
//!     to be_some {
//!         equal(Some(1)),
//!         be_some
//!     }
//! }
//!
//! expect(None as Option<String>) {
//!     to be_none {
//!         equal(None),
//!         be_none
//!     }
//! }
//!
//! expect(Ok(1u8) as Result<u8, ()>) {
//!     to be_ok
//! }
//!
//! expect(Err(()) as Result<String, ()>) {
//!     to be_err
//! }
//! # }
//! # }
//! # tests::expect_some_one_as_option::to_be_some().unwrap();
//! # tests::expect_none_as_option::to_be_none().unwrap();
//! # tests::expect_ok_one_as_result::to_be_ok().unwrap();
//! # tests::expect_err__as_result::to_be_err().unwrap();
//! ```
//!
//! ## Custom assertions
//!
//! Let's Expect provides a way to define custom assertions. An assertion is a function that takes the reference to the
//! subject and returns an [`AssertionResult`](../lets_expect_core/assertions/assertion_result/index.html).
//!
//! Here's two custom assertions:
//!
//! ```
//! # #[derive(Clone)]
//! # pub struct Point {
//! #     pub x: i32,
//! #     pub y: i32,
//! # }
//! use lets_expect::*;
//!
//! fn have_positive_coordinates(point: &Point) -> AssertionResult {
//!     if point.x > 0 && point.y > 0 {
//!         Ok(())
//!     } else {
//!         Err(AssertionError::new(vec![format!(
//!             "Expected ({}, {}) to be positive coordinates",
//!             point.x, point.y
//!         )]))
//!     }
//! }
//!
//! fn have_x_coordinate_equal(x: i32) -> impl Fn(&Point) -> AssertionResult {
//!     move |point: &Point| {
//!         if point.x == x {
//!             Ok(())
//!         } else {
//!             Err(AssertionError::new(vec![format!(
//!                 "Expected x coordinate to be {}, but it was {}",
//!                 x, point.x
//!             )]))
//!         }
//!     }
//! }
//! ```
//!
//! And here's how to use them:
//!
//! ```
//! # mod tests {
//! # use lets_expect::lets_expect;
//! # #[derive(Clone)]
//! # pub struct Point {
//! #     pub x: i32,
//! #     pub y: i32,
//! # }
//! # fn have_positive_coordinates(point: &Point) -> AssertionResult {
//! #     if point.x > 0 && point.y > 0 {
//! #         Ok(())
//! #     } else {
//! #         Err(AssertionError::new(vec![format!(
//! #             "Expected ({}, {}) to be positive coordinates",
//! #             point.x, point.y
//! #         )]))
//! #     }
//! # }
//! # fn have_x_coordinate_equal(x: i32) -> impl Fn(&Point) -> AssertionResult {
//! #     move |point| {
//! #         if point.x == x {
//! #             Ok(())
//! #         } else {
//! #             Err(AssertionError::new(vec![format!(
//! #                 "Expected x coordinate to be {}, but it was {}",
//! #                 x, point.x
//! #             )]))
//! #         }
//! #     }
//! # }
//! # lets_expect! { #method
//! expect(Point { x: 2, y: 22 }) {
//!     to have_valid_coordinates {
//!         have_positive_coordinates,
//!         have_x_coordinate_equal(2)
//!     }
//! }
//! # }
//! # }
//! # tests::expect_point::to_have_valid_coordinates().unwrap();
//! ```
//!
//! Remember to import your custom assertions in your test module.
//!
//! ## Custom change assertions
//!
//! Similarly custom change assertions can be defined:
//!
//! ```
//! use lets_expect::*;
//!
//! fn by_multiplying_by(x: i32) -> impl Fn(&i32, &i32) -> AssertionResult {
//!     move |before, after| {
//!         if *after == *before * x {
//!             Ok(())
//!         } else {
//!             Err(AssertionError::new(vec![format!(
//!                 "Expected {} to be multiplied by {} to be {}, but it was {} instead",
//!                 before,
//!                 x,
//!                 before * x,
//!                 after
//!             )]))
//!         }
//!     }
//! }
//! ```
//!
//! And used like so:
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # fn by_multiplying_by(x: i32) -> impl Fn(&i32, &i32) -> AssertionResult {
//! #     move |before, after| {
//! #         if *after == *before * x {
//! #             Ok(())
//! #         } else {
//! #             Err(AssertionError::new(vec![format!(
//! #                 "Expected {} to be multiplied by {} to be {}, but it was {} instead",
//! #                 before,
//! #                 x,
//! #                 before * x,
//! #                 after
//! #             )]))
//! #         }
//! #     }
//! # }
//! # lets_expect! { #method
//! expect(a *= 5) {
//!     let mut a = 5;
//!
//!     to change(a.clone()) by_multiplying_by(5)
//! }
//! # }
//! # }
//! # tests::expect_a_multiply_equal_five::to_change_a_clone_by_multiplying_by_five().unwrap();
//! ```
//!
//! ## `before` and `after`
//!
//! The contents of the `before` blocks are executed before the subject is evaluated, but after the `let` bindings are executed. The contents of the `after` blocks are executed
//! after the subject is evaluated and the assertions are verified.
//!
//! `before` blocks are run in the order they are defined. Parent `before` blocks being run before child `before` blocks. The reverse is true for `after` blocks.
//! `after` blocks are guaranteed to run even if assertions fail. They however will not run if the let statements, before blocks, subject evaluation or assertions panic.
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method
//! let mut messages: Vec<&str> = Vec::new();
//! before {
//!     messages.push("first message");
//! }
//! after {
//!     messages.clear();
//! }
//! expect(messages.len()) { to equal(1) }
//! expect(messages.push("new message")) {
//!     to change(messages.len()) { from(1), to(2) }
//! }
//! # }
//! # }
//! # tests::expect_messages_len::to_equal_one().unwrap();
//! # tests::expect_messages_push_string::to_change_messages_len_from_one().unwrap();
//! ```
//!
//! ## `panic!`
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # struct IPanic {
//! #     pub should_panic: bool,
//! # }
//! # impl IPanic {
//! #     pub fn new() -> Self {
//! #         Self {
//! #             should_panic: false,
//! #         }
//! #     }
//! #     pub fn panic_if_should(&self) {
//! #         if self.should_panic {
//! #             panic!();
//! #         }
//! #     }
//! # }
//! # lets_expect! { #method
//! expect(panic!("I panicked!")) {
//!     to panic
//! }
//!
//! expect(2) {
//!     to not_panic
//! }
//!
//! expect(i_panic.should_panic = true) {
//!     let mut i_panic = IPanic::new();
//!     to change(i_panic.panic_if_should()) { from_not_panic, to_panic }
//! }
//! # }
//! # }
//! # tests::expect_panic::to_panic().unwrap();
//! # tests::expect_two::to_not_panic().unwrap();
//! # tests::expect_i_panic_should_panic_is_true::to_change_i_panic_panic_if_should_from_not_panic().unwrap();
//! ```
//!
//! ## Numbers
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method
//! expect(2.1) {
//!    to be_close_to(2.0, 0.2)
//!    to be_greater_than(2.0)
//!    to be_less_or_equal_to(2.1)
//! }
//! # }
//! # }
//! # tests::expect_two_point_ten::to_be_close_to_two_point_zero_zero_point_twenty().unwrap();
//! # tests::expect_two_point_ten::to_be_greater_than_two_point_zero().unwrap();
//! # tests::expect_two_point_ten::to_be_less_or_equal_to_two_point_ten().unwrap();
//! ```
//!
//! ## Iterators
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method
//! expect(vec![1, 2, 3]) {
//!    to have(mut iter()) all(be_greater_than(0))
//!    to have(mut iter()) any(be_greater_than(2))
//! }
//! # }
//! # }
//! # tests::expect_vec::to_have_mut_iter_all_be_greater_than_zero().unwrap();
//! # tests::expect_vec::to_have_mut_iter_any_be_greater_than_two().unwrap();
//! ```
//!
//! ## Mutable variables and references
//!
//! For some tests you may need to make the tested value mutable or you may need to pass a mutable reference to the assertions. In `expect`, `have` and `make` you can
//! use the `mut` keyword to do that.
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method
//! expect(mut vec![1, 2, 3]) { // make the subject mutable
//!     to have(remove(1)) equal(2)
//! }
//!
//! expect(mut vec.iter()) { // pass a mutable reference to the iterator to the assertion
//!     let vec = vec![1, 2, 3];
//!     to all(be_greater_than(0))
//! }
//!
//! expect(vec![1, 2, 3]) {
//!     to have(mut iter()) all(be_greater_than(0)) // pass a mutable reference to the iterator to the assertion
//! }
//! # }
//! # }
//! # tests::expect_vec::to_have_mut_iter_all_be_greater_than_zero().unwrap();
//! # tests::expect_mut_vec_iter::to_all_be_greater_than_zero().unwrap();
//! # tests::expect_mut_vec::to_have_remove_one_equal_two().unwrap();
//! ```
//!
//! `let` and `when` statements also support `mut`.
//!
//! ## Explicit identifiers for `expect` and `when`
//!
//! Because Let's Expect uses standard Rust tests under the hood it has to come up with a unique identifier for each test. To make those identifiers
//! readable Let's Expect uses the expressions in `expect` and `when` to generate the name. This works well for simple expressions but can get a bit
//! messy for more complex expressions. Sometimes it can also result in duplicated names. To solve those issues you can use the `as` keyword to give
//! the test an explicit name:
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method
//! expect(a + b + c) as sum_of_three {
//!     when(a = 1, b = 1, c = 1) as everything_is_one to equal(3)
//! }
//! # }
//! # }
//! # tests::expect_sum_of_three::when_everything_is_one::to_equal_three().unwrap();
//! ```
//!
//! This will create a test_named:
//! ```text
//! expect_sum_of_three::when_everything_is_one::to_equal_three
//! ```
//!
//! instead of
//!
//! ```text
//! expect_a_plus_b_plus_c::when_a_is_one_b_is_one_c_is_one::to_equal_three
//! ```
//!
//! ## Stories
//!
//! Let's Expect promotes tests that only test one piece of code at a time. Up until this point all the test we've seen define a subject, run that subject and
//! verify the result. However there can be situations where we want to run and test multiple pieces of code in sequence. This could be for example because executing a piece
//! of code might be time consuming and we want to avoid doing it multiple times in multiple tests.
//!
//! To address this Let's Expect provides the `story` keyword. Stories are a bit more similar to classic tests in that they allow
//! arbitrary statements to be interleaved with assertions.
//!
//! Please note that the `expect` keyword inside stories has to be followed by `to` and can't open a block.
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # struct User {
//! #     name: String,
//! #     password: String
//! # }
//! #
//! # #[derive(Clone, Debug, PartialEq, Eq)]
//! # struct AuthenticationError {
//! #     message: String
//! # }
//! #
//! # struct Page {
//! #     pub logged_in: bool,
//! # }
//! #
//! # impl Page {
//! #     pub fn new() -> Self {
//! #         Self { logged_in: false }
//! #     }
//! #     pub fn login(&mut self, user: &User) -> Result<(), AuthenticationError> {
//! #         if user.name == "valid_name" && user.password == "valid_password" {
//! #             self.logged_in = true;
//! #             Ok(())
//! #         } else {
//! #             Err(AuthenticationError { message: "Invalid credentials".to_string() })
//! #         }
//! #     }
//! # }
//! # lets_expect! { #method
//! # let mut page = Page::new();
//! #
//! # let invalid_user = User {
//! #     name: "invalid".to_string(),
//! #     password: "invalid".to_string()
//! # };
//! # let valid_user = User {
//! #     name: "valid_name".to_string(),
//! #     password: "valid_password".to_string()
//! # };
//! #
//! story login_is_successful {
//!     expect(page.logged_in) to be_false
//!
//!     let login_result = page.login(&invalid_user);
//!
//!     expect(&login_result) to be_err
//!     expect(&login_result) to equal(Err(AuthenticationError { message: "Invalid credentials".to_string() }))
//!     expect(page.logged_in) to be_false
//!
//!     let login_result = page.login(&valid_user);
//!
//!     expect(login_result) to be_ok
//!     expect(page.logged_in) to be_true
//! }
//! # }
//! # }
//! # tests::login_is_successful().unwrap();
//! ```
//!
//! ## Supported 3rd party libraries
//!
//! ### Tokio
//!
//! Let's Expect works with [Tokio](https://tokio.rs/). To use Tokio in your tests you need to add the `tokio` feature in your `Cargo.toml`:
//!
//! ```toml
//! lets_expect = { version = "*", features = ["tokio"] }
//! ```
//!
//! Then whenever you want to use Tokio in your tests you need to add the `tokio_test` attribute to your `lets_expect!` macros like so:
//!
//! ```
//! # use lets_expect::lets_expect;
//! lets_expect! { #tokio_test
//! }
//! ```
//!
//! This will make Let's Expect use `#[tokio::test]` instead of `#[test]` in generated tests.
//!
//! Here's an example of a test using Tokio:
//!
//! ```
//! # mod tests {
//! # use lets_expect::*;
//! # lets_expect! { #method_async
//! let value = 5;
//! let spawned = tokio::spawn(async move {
//!     value
//! });
//!
//! expect(spawned.await) {
//!     to match_pattern!(Ok(5))
//! }
//! # }
//! # }
//! # tokio_test::block_on(async { tests::expect_await_spawned::to_match_pattern().await.unwrap() });
//! ```
//!
//! # Assertions
//!
//! This library has fairly few builtin assertions compared to other similar ones. This is because the use of `have`, `make` and `match_pattern!` allows for a
//! expressive and flexible conditions without the need for a lot of different assertions.
//!
//! The full list of assertions is available in the [assertions module](../lets_expect_assertions/index.html).
//!
//! # Examples
//!
//! Let's expect repository contains tests that might be useful as examples of using the library.
//! You can find them [here](https://github.com/tomekpiotrowski/lets_expect/tree/main/tests).
//!
//! # Debugging
//!
//! If you're having trouble with your tests you can use [cargo-expand](https://github.com/dtolnay/cargo-expand) to see what code is generated by Let's Expect.
//! The generated code is not always easy to read and is not guaranteed to be stable between versions. Still it can be useful for debugging.
//!
//! # License
//!
//! This project is licensed under the terms of the MIT license.

pub use std::panic;

pub use lets_expect_macro::lets_expect;

pub use lets_expect_core::execution::executed_assertion::ExecutedAssertion;
pub use lets_expect_core::execution::executed_expectation::ExecutedExpectation;
pub use lets_expect_core::execution::executed_test_case::ExecutedTestCase;
pub use lets_expect_core::execution::test_failure::TestFailure;
pub use lets_expect_core::execution::test_result::test_result_from_cases;
pub use lets_expect_core::execution::test_result::TestResult;

pub use lets_expect_core::assertions::assertion_error::AssertionError;
pub use lets_expect_core::assertions::assertion_result::AssertionResult;

pub use lets_expect_assertions::assertions::*;

#[cfg(feature = "tokio")]
pub use tokio;
