use std::ops::Index;
use bevy::prelude::Entity;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::logic::block::{Block, BlockType};
use crate::logic::chunk::ChunkBlockCoord;

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;

/// Сущность описывающая один игровой чанк
pub struct Chunk {
    /// Сущность к которой прикреплен данный чанк
    entity: Entity,
    /// Массив со всеми блоками в чанке
    blocks: [[[Option<Block>; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE],
}

impl Chunk {
    pub fn new(entity: Entity, seed: u32) -> Self {
        let mut rand = StdRng::seed_from_u64(seed as u64);
        let mut blocks: [[[Option<Block>; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE] = Default::default();

        for x in 0..CHUNK_SIZE_USIZE {
            for y in 0..CHUNK_SIZE_USIZE {
                for z in 0..CHUNK_SIZE_USIZE {
                    if z == 0 {
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
            entity,
            blocks,
        }
    }
}

impl Index<&ChunkBlockCoord> for Chunk {
    type Output = Option<Block>;

    fn index(&self, index: &ChunkBlockCoord) -> &Self::Output {
        &self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}