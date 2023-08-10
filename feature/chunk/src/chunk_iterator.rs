use bevy_math::UVec3;
use crate::Chunk;
use crate::chunk::CHUNK_SIZE;
use crate::chunk_block_pos::ChunkBlockPos;

impl<'a, BLOCK, METADATA> IntoIterator for &'a Chunk<BLOCK, METADATA> {
    type Item = (ChunkBlockPos, &'a Option<BLOCK>);
    type IntoIter = ChunkIter<'a, BLOCK, METADATA>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            chunk: self,
            pos: UVec3::default(),
        }
    }
}

pub struct ChunkIter<'a, BLOCK, METADATA> {
    chunk: &'a Chunk<BLOCK, METADATA>,
    pos: UVec3,
}

impl<'a, BLOCK, METADATA> Iterator for ChunkIter<'a, BLOCK, METADATA> {
    type Item = (ChunkBlockPos, &'a Option<BLOCK>);

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