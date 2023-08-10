use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::{Chunk, ChunkPos};
use crate::absolute_block_pos::AbsoluteBlockPos;

/// Карта с чанками
#[derive(Default)]
pub struct ChunkMap<BLOCK, METADATA> {
    chunks: HashMap<ChunkPos, Chunk<BLOCK, METADATA>>,
}

impl<BLOCK, METADATA> ChunkMap<BLOCK, METADATA> {
    /// Возвращает блок по текущим координатам если такой блок существует
    pub fn get_block_at(&self, index: AbsoluteBlockPos) -> &Option<BLOCK> {
        // Ищем чанк по переданным абсолютным координатам блока
        let chunk_pos = ChunkPos::from(index);
        let chunk = &self.chunks.get(&chunk_pos);

        if let Some(chunk) = chunk {
            // Если нашли чанк возвращаем блок в этом чанке
            let local_block_pos = chunk_pos.try_global_pos_into_chunk_pos(index).unwrap();
            &chunk[&local_block_pos]
        } else { &None }
    }
}

impl<BLOCK, METADATA> Deref for ChunkMap<BLOCK, METADATA>
{
    type Target = HashMap<ChunkPos, Chunk<BLOCK, METADATA>>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

impl<BLOCK, METADATA> DerefMut for ChunkMap<BLOCK, METADATA> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chunks
    }
}
