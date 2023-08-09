use bevy_math::UVec3;
use generic_assert::{Assert, IsTrue};
use crate::Chunk;
use crate::chunk_block_pos::ChunkBlockPos;

impl<'a, const CHUNK_SIZE: usize, BLOCK, METADATA> IntoIterator for &'a Chunk<CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Item = (ChunkBlockPos<CHUNK_SIZE>, &'a Option<BLOCK>);
    type IntoIter = ChunkIter<'a, CHUNK_SIZE, BLOCK, METADATA>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            chunk: self,
            pos: UVec3::default(),
        }
    }
}

pub struct ChunkIter<'a, const CHUNK_SIZE: usize, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    chunk: &'a Chunk<CHUNK_SIZE, BLOCK, METADATA>,
    pos: UVec3,
}

impl<'a, const CHUNK_SIZE: usize, BLOCK, METADATA> Iterator for ChunkIter<'a, CHUNK_SIZE, BLOCK, METADATA>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Item = (ChunkBlockPos<CHUNK_SIZE>, &'a Option<BLOCK>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x as usize == CHUNK_SIZE {
            return None;
        }

        let block_coord = self.pos.try_into().unwrap();
        let block = &self.chunk[&block_coord];

        self.pos.z = (self.pos.z + 1) % CHUNK_SIZE as u32;
        if self.pos.z == 0 {
            self.pos.y = (self.pos.y + 1) % CHUNK_SIZE as u32;
            if self.pos.y == 0 {
                self.pos.x = self.pos.x + 1;
            }
        }

        Some((block_coord, block))
    }
}