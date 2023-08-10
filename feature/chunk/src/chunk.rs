use std::ops::{Index, IndexMut};
use crate::chunk_block_pos::ChunkBlockPos;

type Array3<BLOCK, const SIZE: usize> = [[[BLOCK; SIZE]; SIZE]; SIZE];

/// Размер чанка
pub const CHUNK_SIZE: usize = 16;

/// Сущность описывающая один игровой чанк.
///
/// [BLOCK] - тип блоков в чанке.
///
/// [METADATA] - любые дополнительные данные
pub struct Chunk<BLOCK, METADATA> {
    metadata: METADATA,
    blocks: Array3<Option<BLOCK>, CHUNK_SIZE>,
}

impl<BLOCK, METADATA> Chunk<BLOCK, METADATA> {
    pub fn new(metadata: METADATA) -> Self {
        Self {
            metadata,
            blocks: std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| None))),
        }
    }

    pub fn get_metadata(&self) -> &METADATA {
        &self.metadata
    }
}

impl<BLOCK, METADATA> Index<&ChunkBlockPos> for Chunk<BLOCK, METADATA> {
    type Output = Option<BLOCK>;

    fn index(&self, index: &ChunkBlockPos) -> &Self::Output {
        &self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}

impl<BLOCK, METADATA> IndexMut<&ChunkBlockPos> for Chunk<BLOCK, METADATA> {
    fn index_mut(&mut self, index: &ChunkBlockPos) -> &mut Self::Output {
        &mut self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}