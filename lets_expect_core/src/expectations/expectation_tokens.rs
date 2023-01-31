use proc_macro2::TokenStream;

type SingleAssertionTokens = (String, TokenStream);

#[derive(Clone)]
pub enum AssertionTokens {
    Single(SingleAssertionTokens),
    Group(String, String, Box<AssertionTokens>),
    Many(Vec<AssertionTokens>),
}

#[derive(Clone)]
pub(crate) struct ExpectationTokens {
    pub before_subject: TokenStream,
    pub after_subject: TokenStream,
    pub assertions: AssertionTokens,
}
