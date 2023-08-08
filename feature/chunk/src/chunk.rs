use std::ops::Index;
use generic_assert::{Assert, IsTrue};
use crate::chunk_block_pos::ChunkBlockPos;

type Array3<BLOCK, const SIZE: usize> = [[[BLOCK; SIZE]; SIZE]; SIZE];

/// Сущность описывающая один игровой чанк.
///
/// [CHUNK_SIZE] - размер стороны чанка (чанк всегда представляет собой куб).
///
/// [BLOCK] - тип блоков в чанке.
///
/// [METADATA] - любые дополнительные данные
pub struct Chunk<const CHUNK_SIZE: usize, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE < u8::MAX as usize }>: IsTrue {
    metadata: METADATA,
    blocks: Array3<Option<BLOCK>, CHUNK_SIZE>,
}

impl<const CHUNK_SIZE: usize, BLOCK, METADATA> Chunk<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE < u8::MAX as usize }>: IsTrue {
    pub fn new(blocks: Array3<Option<BLOCK>, CHUNK_SIZE>, metadata: METADATA) -> Self {
        Self {
            metadata,
            blocks,
        }
    }

    pub fn get_metadata(&self) -> &METADATA {
        &self.metadata
    }
}

impl<const CHUNK_SIZE: usize, BLOCK, METADATA> Index<&ChunkBlockPos> for Chunk<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE < u8::MAX as usize }>: IsTrue {
    type Output = Option<BLOCK>;

    fn index(&self, index: &ChunkBlockPos) -> &Self::Output {
        &self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}