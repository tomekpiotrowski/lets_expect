use proc_macro2::{Ident, TokenStream};

use super::{story::Story, runtime::Runtime};

pub struct StoryBlock {
    pub keyword: Ident,
    pub story: Story
}

impl StoryBlock {
    pub fn new(keyword: Ident, story: Story) -> Self {
        StoryBlock { keyword, story }
    }

    pub fn to_tokens(&self, runtime: &Runtime) -> TokenStream {
        self.story.to_tokens(runtime)
    }
}
