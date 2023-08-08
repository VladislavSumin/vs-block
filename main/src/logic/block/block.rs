use crate::logic::block::BlockType;

#[derive(Default)]
pub struct Block {
    block_type: BlockType,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}
