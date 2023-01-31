![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/tomekpiotrowski/lets_expect/build.yaml?branch=main)
![Crates.io](https://img.shields.io/crates/v/lets_expect)
![GitHub](https://img.shields.io/github/license/tomekpiotrowski/lets_expect)

# Let's Expect

<!-- cargo-rdme start -->

Clean tests in Rust.

```rust
expect(a + 2) {
    when(a = 2) {
        to equal(4)
    }
}
```

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Usage](#usage)
    * [How does it work?](#how-does-it-work)
    * [Where to put my tests?](#where-to-put-my-tests)
    * [`expect` and `to`](#expect-and-to)
    * [`let`](#let)
    * [`when`](#when)
    * [`have`](#have)
    * [`make`](#make)
    * [`change`](#change)
    * [`before` and `after`](#before-and-after)
    * [Explicit identifiers for `expect` and `when`](#explicit-identifiers-for-expect-and-when)
    * [Stories](#stories)
    * [Mutable variables and references](#mutable-variables-and-references)
4. [Assertions](#assertions)
    * [`bool`](#bool)
    * [`equality`](#equality)
    * [Numbers](#numbers)
    * [`match_pattern!`](#match_pattern)
    * [`Option` and `Result`](#option_and_result)
    * [`panic`](#panic)
    * [Iterators](#iterators)
    * [Custom assertions](#custom-assertions)
    * [Custom `change` assertions](#custom-change-assertions)
    * [Assertions module](#assertions-module)
5. [Supported libraries](#supported-libraries)
    * [Tokio](#tokio)
6. [More examples](#more-examples)
7. [Known issues and limitations](#known-issues-and-limitations)
8. [Debugging](#debugging)
9. [License](#license)

### Introduction

How often when you see a Rust test you think to yourself "wow, this is a really beautifully written test"? Not often, right?
Classic Rust tests do not provide any structure beyond the test function itself. This often results in a lot of boilerplate code, ad-hoc test structure and overall
poor quality.

Tests are about verifying that a given piece of code run under certain conditions works as expected. A good testing framework embraces this way of thinking.
It makes it easy to structure your code in a way that reflects it. Folks in other communities have been doing this for a long time with tools like
[RSpec](https://relishapp.com/rspec) and [Jasmine](https://jasmine.github.io/).

If you want beautiful, high-quality tests that are a pleasure to read and write you need something else. Using Rust's procedural macros `lets_expect` introduces
a syntax that let's you clearly state **what** you're testing, under what **conditions** and what is the **expected result**.

The outcome is:
* easy to read, DRY, TDD-friendly tests
* less boilerplate, less code
* nicer error messages
* more fun

#### Example

```rust
expect(posts.create_post(title, category_id)) {
    before { posts.push(Post {title: "Post 1" }) }
    after { posts.clear() }

    when(title = valid_title) {
        when(category_id = valid_category) to create_a_post {
            be_ok,
            have(as_ref().unwrap().title) equal(valid_title),
            change(posts.len()) { from(1), to(2) }
        }

        when(category_id = invalid_category) to return_an_error {
            be_err,
            have(as_ref().unwrap_err().message) equal("Invalid category"),
            not_change(posts.len())
        }
    }

    when(title = invalid_title, category_id = valid_category) to be_err
}
```

Now let's compare it to a classic Rust test that does the same thing:


```rust
fn run_setup<T>(test: T) -> ()
where T: FnOnce(&mut Posts) -> () + panic::UnwindSafe
{
    let mut posts = Posts { posts: vec![] };
    posts.push(Post { title: "Post 1" });
    let posts = Mutex::new(posts);
    let result = panic::catch_unwind(|| {
        test(posts.try_lock().unwrap().deref_mut())
    });
    
    posts.try_lock().unwrap().clear();
    assert!(result.is_ok());
}

#[test]
fn creates_a_post() {
    run_setup(|posts: &mut Posts| {
        let before_count = posts.len();
        let result = posts.create_post(VALID_TITLE, VALID_CATEGORY);
        let after_count = posts.len();
        assert!(result.is_ok());
        assert_eq!(VALID_TITLE, result.unwrap().title);
        assert_eq!(after_count - before_count, 1);
    })
}

#[test]
fn returns_an_error_when_category_is_invalid() {
    run_setup(|posts: &mut Posts| {
        let before_count = posts.len();
        let result = posts.create_post(VALID_TITLE, INVALID_CATEGORY);
        let after_count = posts.len();
        assert!(result.is_err());
        assert_eq!("Invalid category", result.unwrap_err().message);
        assert_eq!(after_count, before_count);
    })
}

#[test]
fn returns_an_error_when_title_is_empty() {
    run_setup(|posts: &mut Posts| {
        let result = posts.create_post("", VALID_CATEGORY);
        assert!(result.is_err());
    })
}

```

### Installation

Add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
lets_expect = "0"
```

### Usage

#### How does it work?

Under the hood `lets_expect` generates a single classic test function for each `to` block. It names those tests automatically based on what you're testing and
organizes those tests into modules. This means you can run those tests using `cargo test` and you can use all `cargo test` features. IDE extensions will
also work as expected.

#### Where to put my tests?

`lets_expect` tests need to be placed inside of a `lets_expect!` macro, which in turn needs to be placed inside of a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use lets_expect::lets_expect;

    lets_expect! {
        expect(subject) {
            to expectation
        }
    }
}
```

It might be a good idea to define a code snippet in your IDE to avoid having to type this piece of boilerplate every time.

The examples here omit the macro for brevity.

#### `expect` and `to`

`expect` sets the subject of the test. It can be any Rust expression (including a block). `to` introduces expectations. It can be followed
by a single expectation or a block of expectations. In the latter case you must provide a name for the test, which needs to be a valid Rust identifier.

```rust
expect(2) {
    to equal(2)
}
```

If there are multiple assertions in a `to` block they need to be separated by a comma.

```rust
expect({ 1 + 1 }) {
    to be_actually_2 {
        equal(2),
        not_equal(3)
    }
}
```

One `to` block generates a single test. This means the subject will be executed once and then all the assertions inside that `to` block will be run.
If you want to generate multiple tests you can use multiple `to` blocks:

```rust
expect(files.create_file()) {
    to make(files.try_to_remove_file()) be_true
    to make(files.file_exists()) be_true
}
```

If your `expect` contains a single item you can omit the braces:

```rust
expect(a + 2) when(a = 2) {
    to equal(4)
}
```


#### `let`

Inside the top level `lets_expect!` macro as well as `expect` and `when` blocks you can use `let` to define variables.

```rust
expect(a) {
    let a = 2;

    to equal(2)
}
```

Variables can be overwritten in nested blocks. New definitions can use values from outer blocks.

```rust
expect(a) {
    let a = 2;

    when a_is_4 {
        let a = a + 2;

        to equal(4)
    }
}
```

Variables don't have to be defined in the order they're used.

```rust
expect(sum) {
    let sum = a + b;
    let a = 2;

    when b_is_three {
        let b = 3;

        to equal(5)
    }
}
```

#### `when`

`when` sets a value of one or more variables for a given block. This keyword is this library's secret sauce. It allows you to define values of variables
for multiples tests in a concise and readable way, without having to repeat it in every test.

```rust
expect(a + b + c) {
    let a = 2;

    when(c = 5) {
        when(b = 3) {
            to equal(10)
        }

        when(a = 10, b = 10) {
            to equal(25)
        }
    }
}
```

You can use similar syntax as in `let` to define variables. The only difference being the `let` keyword itself is ommited.

```rust
expect(a += 1) {
    when(mut a: i64 = 1) {
        to change(a.clone()) { from(1), to(2) }
    }
}
```

You can also use `when` with an identifier. This will simply create a new context with the given identifier. No new variables are defined.

```rust
expect(login(username, password)) {
    when credentials_are_invalid {
        let username = "invalid";
        let password = "invalid";

        to be_false
    }
}
```

If your `when` contains only one item the braces can be ommited:

```rust
expect(a + 2) when(a = 2) to equal(4)
```

`when` blocks do not have to be placed inside of `expect` blocks. Their order can be reversed.

```rust
when(a = 2) {
  expect(a + 2) to equal(4)
}
```

#### `have`

`have` is used to test values of attributes or return values of methods of the subject.

```rust
let response = Response { status: 200, content: ResponseContent::new("admin", "123") };

expect(response) {
    to be_valid {
        have(status) equal(200),
        have(is_ok()) be_true,
        have(content) {
            have(username) equal("admin".to_string()),
            have(token) equal("123".to_string()),
        }
    }
}
```

Multiple assertions can be provided to `have` by wrapping them in curly braces and separating them with commas.

#### `make`

`make` is used to test values of arbitrary expressions.

```rust
expect(posts.push((user_id, "new post"))) {
    let user_id = 1;

    to make(user_has_posts(user_id)) be_true
}
```

Multiple assertions can be provided to `make` by wrapping them in curly braces and separating them with commas.

#### `change`

`change` is used to test if and how a value changes after subject is executed. The expression given as an argument to `change` is evaluated twice. Once before the subject is executed and once after.
The two values are then provided to the assertions specified in the `change` block.

```rust
expect(posts.create_post(title, category_id)) {
    after { posts.clear() }

    when(title = valid_title) {
        when(category_id = valid_category) {
            to change(posts.len()) { from(0), to(1) }
        }

        when(category_id = invalid_category) {
            to not_change(posts.len())
        }
    }
}
```

#### `before` and `after`

The contents of the `before` blocks are executed before the subject is evaluated, but after the `let` bindings are executed. The contents of the `after` blocks are executed
after the subject is evaluated and the assertions are verified.

`before` blocks are run in the order they are defined. Parent `before` blocks being run before child `before` blocks. The reverse is true for `after` blocks.
`after` blocks are guaranteed to run even if assertions fail. They however will not run if the let statements, before blocks, subject evaluation or assertions panic.

```rust
let mut messages: Vec<&str> = Vec::new();
before {
    messages.push("first message");
}
after {
    messages.clear();
}
expect(messages.len()) { to equal(1) }
expect(messages.push("new message")) {
    to change(messages.len()) { from(1), to(2) }
}
```

#### Explicit identifiers for `expect` and `when`

Because `lets_expect` uses standard Rust tests under the hood it has to come up with a unique identifier for each test. To make those identifiers
readable `lets_expect` uses the expressions in `expect` and `when` to generate the name. This works well for simple expressions but can get a bit
messy for more complex expressions. Sometimes it can also result in duplicated names. To solve those issues you can use the `as` keyword to give
the test an explicit name:

```rust
expect(a + b + c) as sum_of_three {
    when(a = 1, b = 1, c = 1) as everything_is_one to equal(3)
}
```

This will create a test_named:
```text
expect_sum_of_three::when_everything_is_one::to_equal_three
```

instead of

```text
expect_a_plus_b_plus_c::when_a_is_one_b_is_one_c_is_one::to_equal_three
```

#### Stories

`lets_expect` promotes tests that only test one piece of code at a time. Up until this point all the test we've seen define a subject, run that subject and
verify the result. However there can be situations where we want to run and test multiple pieces of code in sequence. This could be for example because executing a piece
of code might be time consuming and we want to avoid doing it multiple times in multiple tests.

To address this `lets_expect` provides the `story` keyword. Stories are a bit more similar to classic tests in that they allow
arbitrary statements to be interleaved with assertions.

Please note that the `expect` keyword inside stories has to be followed by `to` and can't open a block.

```rust
story login_is_successful {
    expect(page.logged_in) to be_false

    let login_result = page.login(&invalid_user);

    expect(&login_result) to be_err
    expect(&login_result) to equal(Err(AuthenticationError { message: "Invalid credentials".to_string() }))
    expect(page.logged_in) to be_false

    let login_result = page.login(&valid_user);

    expect(login_result) to be_ok
    expect(page.logged_in) to be_true
}
```

>
> **NOTE:**  For now `expect` blocks can't be placed inside of loops or closures. They need to be top-level items in a story.
>

#### Mutable variables and references

For some tests you may need to make the tested value mutable or you may need to pass a mutable reference to the assertions. In `expect`, `have` and `make` you can
use the `mut` keyword to do that.

```rust
expect(mut vec![1, 2, 3]) { // make the subject mutable
    to have(remove(1)) equal(2)
}

expect(mut vec.iter()) { // pass a mutable reference to the iterator to the assertion
    let vec = vec![1, 2, 3];
    to all(be_greater_than(0))
}

expect(vec![1, 2, 3]) {
    to have(mut iter()) all(be_greater_than(0)) // pass a mutable reference to the iterator to the assertion
}
```

`let` and `when` statements also support `mut`.


### Assertions

#### `bool`

```rust
expect(2 == 2) to be_true
expect(2 != 2) to be_false
```

#### `equality`

```rust
expect(2) to be_actually_two {
  equal(2),
  not_equal(3)
}
```

#### Numbers

```rust
expect(2.1) {
   to be_close_to(2.0, 0.2)
   to be_greater_than(2.0)
   to be_less_or_equal_to(2.1)
}
```

#### `match_pattern!`

`match_pattern!` is used to test if a value matches a pattern. It's functionality is similar to [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro.

```rust
expect(Response::UserCreated) {
    to match_pattern!(Response::UserCreated)
}

expect(Response::ValidationFailed("email")) {
    to match_email {
        match_pattern!(Response::ValidationFailed("email")),
        not_match_pattern!(Response::ValidationFailed("email2"))
    }
}
```

#### `Option` and `Result`

`lets_expect` provides a set of assertions for `Option` and `Result` types.

```rust
expect(Some(1u8) as Option<u8>) {
    to be_some_and equal(1)

    to be_some {
        equal(Some(1)),
        be_some
    }
}

expect(None as Option<String>) {
    to be_none {
        equal(None),
        be_none
    }
}

expect(Ok(1u8) as Result<u8, ()>) {
    to be_ok_and equal(1)

    to be_ok {
        be_ok,
        equal(Ok(1)),
    }
}

expect(Err(2) as Result<(), i32>) {
    to be_err_and equal(2)

    to be_err {
        be_err,
        equal(Err(2)),
    }
}
```

#### `panic!`

```rust
expect(panic!("I panicked!")) {
    to panic
}

expect(2) {
    to not_panic
}
```

`panic` and `not_panic` assertions can be the only assertions present in a `to` block.


#### Iterators

```rust
expect(vec![1, 2, 3]) {
   to have(mut iter()) all(be_greater_than(0))
   to have(mut iter()) any(be_greater_than(2))
}
```

#### Custom assertions

`lets_expect` provides a way to define custom assertions. An assertion is a function that takes the reference to the
subject and returns an [`AssertionResult`](../lets_expect_core/assertions/assertion_result/index.html).

Here's two custom assertions:

```rust
use lets_expect::*;

fn have_positive_coordinates(point: &Point) -> AssertionResult {
    if point.x > 0 && point.y > 0 {
        Ok(())
    } else {
        Err(AssertionError::new(vec![format!(
            "Expected ({}, {}) to be positive coordinates",
            point.x, point.y
        )]))
    }
}

fn have_x_coordinate_equal(x: i32) -> impl Fn(&Point) -> AssertionResult {
    move |point: &Point| {
        if point.x == x {
            Ok(())
        } else {
            Err(AssertionError::new(vec![format!(
                "Expected x coordinate to be {}, but it was {}",
                x, point.x
            )]))
        }
    }
}
```

And here's how to use them:

```rust
expect(Point { x: 2, y: 22 }) {
    to have_valid_coordinates {
        have_positive_coordinates,
        have_x_coordinate_equal(2)
    }
}
```

Remember to import your custom assertions in your test module.

#### Custom change assertions

Similarly custom change assertions can be defined:

```rust
use lets_expect::*;

fn by_multiplying_by(x: i32) -> impl Fn(&i32, &i32) -> AssertionResult {
    move |before, after| {
        if *after == *before * x {
            Ok(())
        } else {
            Err(AssertionError::new(vec![format!(
                "Expected {} to be multiplied by {} to be {}, but it was {} instead",
                before,
                x,
                before * x,
                after
            )]))
        }
    }
}
```

And used like so:

```rust
expect(a *= 5) {
    let mut a = 5;

    to change(a.clone()) by_multiplying_by(5)
}
```

#### Assertions

This library has fairly few builtin assertions compared to other similar ones. This is because the use of `have`, `make` and `match_pattern!` allows for
expressive and flexible conditions without the need for a lot of different assertions.

The full list of assertions is available in the [assertions module](https://docs.rs/lets_expect_assertions).



### Supported libraries

#### Tokio

`lets_expect` works with [Tokio](https://tokio.rs/). To use Tokio in your tests you need to add the `tokio` feature in your `Cargo.toml`:

```toml
lets_expect = { version = "*", features = ["tokio"] }
```

Then whenever you want to use Tokio in your tests you need to add the `tokio_test` attribute to your `lets_expect!` macros like so:

```rust
lets_expect! { #tokio_test
}
```

This will make `lets_expect` use `#[tokio::test]` instead of `#[test]` in generated tests.

Here's an example of a test using Tokio:

```rust
let value = 5;
let spawned = tokio::spawn(async move {
    value
});

expect(spawned.await) {
    to match_pattern!(Ok(5))
}
```


### More examples

`lets_expect` repository contains tests that might be useful as examples of using the library.
You can find them [here](https://github.com/tomekpiotrowski/lets_expect/tree/main/tests).

### Known issues and limitations

* rust-analyzer's auto-import doesn't seem to work well from inside of macros. It might be necessary to manually add `use` statements for types from outside of the module.
* Syntax highlighting doesn't work with `lets_expect` syntax. Currently there's no way for Rust macros to export their syntax to language tools.
* Shared contexts (similar to [RSpec](https://relishapp.com/rspec/rspec-core/docs/example-groups/shared-context)) seem to be impossible to implement without
  [eager macro expansion](https://rustc-dev-guide.rust-lang.org/macro-expansion.html#eager-expansion).

### Debugging

If you're having trouble with your tests you can use [cargo-expand](https://github.com/dtolnay/cargo-expand) to see what code is generated by `lets_expect`.
The generated code is not always easy to read and is not guaranteed to be stable between versions. Still it can be useful for debugging.

### License

This project is licensed under the terms of the MIT license.

<!-- cargo-rdme end -->
