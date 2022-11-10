use syn::Block;

use super::keyword;

pub struct BeforeBlock {
    pub keyword: keyword::before,
    pub before: Block,
}

impl BeforeBlock {
    pub fn new(keyword: keyword::before, block: Block) -> BeforeBlock {
        BeforeBlock {
            keyword,
            before: block,
        }
    }
}
