use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use crate::world::block::{Block, BlockType};

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;

/// Сущность описывающая один игровой чанк
pub struct Chunk {
    // temp solution
    pub blocks: [[[Option<Block>; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE],
}

impl Chunk {
    pub fn new(seed: u32) -> Self {
        let mut rand = StdRng::seed_from_u64(seed as u64);
        let mut blocks: [[[Option<Block>; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE] = Default::default();

        for x in 0..CHUNK_SIZE_USIZE {
            for y in 0..CHUNK_SIZE_USIZE {
                for z in 0..CHUNK_SIZE_USIZE {
                    if y == 0 {
                        blocks[x][y][z] = Some(Block::new(BlockType::BEDROCK));
                        continue;
                    }
                    let gen_block = rand.gen_bool(0.2);
                    if gen_block {
                        blocks[x][y][z] = Some(Block::new(BlockType::GRASS));
                    }
                }
            }
        }

        Self {
            blocks
        }
    }
}