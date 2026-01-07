use syn::Block;

use super::keyword;

pub struct BeforeBlock {
    pub _keyword: keyword::before,
    pub before: Block,
}

impl BeforeBlock {
    pub fn new(keyword: keyword::before, block: Block) -> Self {
        Self {
            _keyword: keyword,
            before: block,
        }
    }
}
