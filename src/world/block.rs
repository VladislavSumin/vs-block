#[derive(Default)]
pub enum BlockType {
    #[default]
    AIR,
    BEDROCK,
    GRASS,
}

#[derive(Default)]
pub struct Block {
    block_type: BlockType,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}
