![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tomekpiotrowski/lets_expect/Build)
![Crates.io](https://img.shields.io/crates/v/lets_expect)
![GitHub](https://img.shields.io/github/license/tomekpiotrowski/lets_expect)

# Let's Expect

<!-- cargo-rdme start -->

Clean tests in Rust.

## Why do I need this? Isn't libtest already good enough?

How often when you see a Rust test you think to yourself "wow, this is a really beautifully written test"? Not often, right?
Classic Rust tests do not provide any structure beyond the test function itself. This often results in a lot of boilerplate code, ad-hoc test structure and overall
poor quality.

Tests are about verifying that a given piece of code run under certain conditions works as expected. A good testing framework embraces this way of thinking.
It makes it easy to structure your code in a way that reflects it. Folks in other communities have been doing this for a long time with tools like
[RSpec](https://relishapp.com/rspec) and [Jasmine](https://jasmine.github.io/).

If you want beautiful, high-quality tests that are a pleasure to read and write you need something else. Using Rust's procedural macros `lets_expect` introduces
a syntax that let's you clearly state **what** you're testing, under what **conditions** and what is the **expected result**:

```rust
expect(a + 2) {
    when(a = 2) {
        to equal(4)
    }
}
```

The outcome is:
* easy to read, DRY, TDD-friendly tests
* less boilerplate, less code
* nicer error messages
* more fun

## Non-trivial example

```rust
expect(posts.create_post(title, category_id)) {
    before { posts.push(Post {title: "Post 1" }) }
    after { posts.clear() }

    when(title = valid_title) {
        when(category_id = valid_category) {
            to create_a_post {
                be_ok,
                have(as_ref().unwrap().title) equal(valid_title),
                change(posts.len()) { from(1), to(2) }
            }
        }

        when(category_id = invalid_category) {
            to return_an_error {
                be_err,
                have(as_ref().unwrap_err().message) equal("Invalid category"),
                not_change(posts.len())
            }
        }
    }

    when(title = invalid_title; category_id = valid_category) { to be_err }
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

## Installation

Add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
lets_expect = "*"
```

## Guide

### Introduction

Under the hood `lets_expect` generates a single classic test function for each `to` block. It names those tests based on assertions present in the test and
organizes those tests into modules. This means you can run those tests using `cargo test` and you can use all `cargo test` features. IDE extensions will
also work as expected.

Let's Expect tests need to be placed inside of a `lets_expect!` macro, which in turn needs to be placed inside of a `tests` module:

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

The examples below omit the macro for brevity.

### `expect` and `to`

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

One `to` block generates a single test. If you want to generate multiple tests you can use multiple `to` blocks:

```rust
expect(files.create_file()) {
    to make(files.try_to_remove_file()) be_true
    to make(files.file_exists()) be_true
}
```

### `let`

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

### `when`

`when` sets a value of one or more variable for a given block. This keyword is this library's secret sauce. It allows you to define values of variables
for multiples tests in a concise and readable way, without having to repeat it in every test.

```rust
expect(a + b + c) {
    let a = 2;

    when(c = 5) {
        when(b = 3) {
            to equal(10)
        }

        when(a = 10; b = 10) {
            to equal(25)
        }
    }
}
```

You can use similar syntax as in `let` to define variables. The only difference being the `let` keyword itself is ommited.

```rust
expect(a += 1) {
    when(mut a: i64 = 1) {
        to change(a) { from(1), to(2) }
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

### `have`

`have` is used to test values of attributes or return values of methods of the subject.

```rust
let response = Response { status: 200 };

expect(response) {
    to be_valid {
        have(status) equal(200),
        have(is_ok()) be_true
    }
}
```

Multiple assertions can be provided to `have` by wrapping them in curly braces and separating them with commas.

### `make`

`make` is used to test values of arbitrary expressions.

```rust
expect(posts.push((user_id, "new post"))) {
    let user_id = 1;

    to make(user_has_posts(user_id)) be_true
}
```

Multiple assertions can be provided to `make` by wrapping them in curly braces and separating them with commas.

### `change`

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

### `match_pattern!`

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

### `Option` and `Result`

Let's Expect provides a set of assertions for `Option` and `Result` types.

```rust
expect(Some(1u8) as Option<u8>) {
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
    to be_ok
}

expect(Err(()) as Result<String, ()>) {
    to be_err
}
```

### Custom assertion

Let's Expect provides a way to define custom assertions. An assertion is a function that takes the references to the
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

### Custom change assertions

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

    to change(a) by_multiplying_by(5)
}
```

### `before` and `after`

The contents of the `before` blocks are executed before the subject is evaluated, but after the `let` bindings are executed. The contents of the `after` blocks are executed
after the subject is evaluated and the assertions are verified. `after` block is guaranteed to be executed even if the subject evaluation or the assertions fail.

`before` blocks are run in the order they are defined. Parent `before` blocks being run before child `before` blocks. The reverse is true for `after` blocks.

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

### `panic!`

```rust
expect(panic!("I panicked!")) {
    to panic
}

expect(2) {
    to not_panic
}

expect(i_panic.should_panic = true) {
    let mut i_panic = IPanic::new();
    to change(i_panic.panic_if_should()) { from_not_panic, to_panic }
}
```

### Numbers

```rust
expect(2.1) {
   to be_close_to(2.0, 0.2)
   to be_greater_than(2.0)
   to be_less_or_equal_to(2.1)
}
```

### Stories

Let's Expect promotes tests that only test one piece of code at a time. Up until this point all the test we've seen define a subject, run that subject and
verify the result. However there can be situations where we want to run and test multiple pieces of code in sequence. This could be for example because executing a piece
of code might be time consuming and we want to avoid doing it multiple times in multiple tests.

To address this Let's Expect provides the `story` keyword. Stories are a bit more similar to classic tests in that they allow
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

### Supported 3rd party libraries

#### Tokio

Let's Expect works with [Tokio](https://tokio.rs/). To use Tokio in your tests you need to add the `tokio` feature in your `Cargo.toml`:

```toml
lets_expect = { version = "*", features = ["tokio"] }
```

Then whenever you want to use Tokio in your tests you need to add the `tokio_test` attribute to your `lets_expect!` macros like so:

```rust
lets_expect! { #tokio_test
}
```

This will make Let's Expect use `#[tokio::test]` instead of `#[test]` in generated tests.

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

## Assertions

This library has fairly few builtin assertions compared to other similar ones. This is because the use of `have`, `make` and `match_pattern!` allows for a
expressive and flexible conditions without the need for a lot of different assertions.

The full list of assertions is available in the [assertions module](../lets_expect_assertions/index.html).

## Examples

Let's expect repository contains tests that might be useful as examples of using the library.
You can find them [here](https://github.com/tomekpiotrowski/lets_expect/tree/main/lets_expect/tests).

## License

This project is licensed under the terms of the MIT license.

<!-- cargo-rdme end -->
