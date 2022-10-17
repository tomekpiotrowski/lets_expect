use syn::{Block, Ident};

pub struct AfterBlock {
    pub identifier: Ident,
    pub after: Block,
}

impl AfterBlock {
    pub fn new(ident: Ident, block: Block) -> AfterBlock {
        AfterBlock {
            identifier: ident,
            after: block,
        }
    }
}
