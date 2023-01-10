use proc_macro2::TokenStream;

use super::{keyword, runtime::Runtime, story::Story};

pub struct StoryBlock {
    pub keyword: keyword::story,
    pub story: Story,
}

impl StoryBlock {
    pub fn new(keyword: keyword::story, story: Story) -> Self {
        Self { keyword, story }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.story.to_tokens(runtime)
    }
}
