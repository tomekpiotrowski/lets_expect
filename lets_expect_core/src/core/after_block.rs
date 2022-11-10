use syn::Block;

use super::keyword;

pub struct AfterBlock {
    pub keyword: keyword::after,
    pub after: Block,
}

impl AfterBlock {
    pub fn new(keyword: keyword::after, block: Block) -> AfterBlock {
        AfterBlock {
            keyword,
            after: block,
        }
    }
}
