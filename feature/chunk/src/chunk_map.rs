use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use generic_assert::{Assert, IsTrue};
use crate::{Chunk, ChunkPos};
use crate::absolute_block_pos::AbsoluteBlockPos;

/// Карта с чанками
#[derive(Default)]
pub struct ChunkMap<const CHUNK_SIZE: usize, BLOCK, METADATA> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    chunks: HashMap<ChunkPos<CHUNK_SIZE>, Chunk<CHUNK_SIZE, BLOCK, METADATA>>,
}

impl<const CHUNK_SIZE: usize, BLOCK, METADATA> ChunkMap<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {

    /// Возвращает блок по текущим координатам если такой блок существует
    pub fn get_block_at(&self, index: AbsoluteBlockPos) -> &Option<BLOCK> {
        // Ищем чанк по переданным абсолютным координатам блока
        let chunk_pos = ChunkPos::<CHUNK_SIZE>::from(index);
        let chunk = &self.chunks.get(&chunk_pos);

        if let Some(chunk) = chunk {
            // Если нашли чанк возвращаем блок в этом чанке
            let local_block_pos = chunk_pos.try_global_pos_into_chunk_pos(index).unwrap();
            &chunk[&local_block_pos]
        } else { &None }
    }
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
