use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use generic_assert::{Assert, IsTrue};
use crate::{Chunk, ChunkPos};

/// Карта с чанками
#[derive(Default)]
pub struct ChunkMap<const CHUNK_SIZE: usize, BLOCK, METADATA> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    chunks: HashMap<ChunkPos<CHUNK_SIZE>, Chunk<CHUNK_SIZE, BLOCK, METADATA>>,
}

impl<const CHUNK_SIZE: usize, BLOCK, METADATA> Deref for ChunkMap<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Target = HashMap<ChunkPos<CHUNK_SIZE>, Chunk<CHUNK_SIZE, BLOCK, METADATA>>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

impl<const CHUNK_SIZE: usize, BLOCK, METADATA> DerefMut for ChunkMap<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chunks
    }
}
