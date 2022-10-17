use syn::{Block, Ident};

pub struct BeforeBlock {
    pub identifier: Ident,
    pub before: Block,
}

impl BeforeBlock {
    pub fn new(ident: Ident, block: Block) -> BeforeBlock {
        BeforeBlock {
            identifier: ident,
            before: block,
        }
    }
}
