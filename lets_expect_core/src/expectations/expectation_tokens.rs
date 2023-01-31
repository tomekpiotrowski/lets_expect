use proc_macro2::TokenStream;

#[derive(Clone)]
pub struct SingleAssertionTokens {
    pub(crate) expression: String,
    pub(crate) assertion: TokenStream,
}

impl SingleAssertionTokens {
    pub fn new(expression: String, assertion: TokenStream) -> Self {
        Self {
            expression,
            assertion,
        }
    }
}

#[derive(Clone)]
pub struct GroupAssertionTokens {
    pub(crate) label: String,
    pub(crate) argument: String,
    pub(crate) guard: Option<TokenStream>,
    pub(crate) context: Option<TokenStream>,
    pub(crate) inner: Box<AssertionTokens>,
}

impl GroupAssertionTokens {
    pub fn new(
        label: String,
        expression: String,
        guard: Option<TokenStream>,
        context: Option<TokenStream>,
        inner: AssertionTokens,
    ) -> Self {
        Self {
            label,
            argument: expression,
            guard,
            context,
            inner: Box::new(inner),
        }
    }
}

#[derive(Clone)]
pub enum AssertionTokens {
    Single(SingleAssertionTokens),
    Group(GroupAssertionTokens),
    Many(Vec<AssertionTokens>),
}

#[derive(Clone)]
pub(crate) struct ExpectationTokens {
    pub before_subject_evaluation: TokenStream,
    pub assertions: AssertionTokens,
}
