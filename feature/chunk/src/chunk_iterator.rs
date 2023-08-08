use crate::Chunk;
use crate::chunk_block_pos::ChunkBlockPos;

impl<'a, const CHUNK_SIZE: usize, BLOCK, METADATA> IntoIterator for &'a Chunk<CHUNK_SIZE, BLOCK, METADATA> {
    type Item = (ChunkBlockPos, &'a Option<BLOCK>);
    type IntoIter = ChunkIter<'a, CHUNK_SIZE, BLOCK, METADATA>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            chunk: self,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

pub struct ChunkIter<'a, const CHUNK_SIZE: usize, BLOCK, METADATA> {
    chunk: &'a Chunk<CHUNK_SIZE, BLOCK, METADATA>,
    x: u8,
    y: u8,
    z: u8,
}

impl<'a, const CHUNK_SIZE: usize, BLOCK, METADATA> Iterator for ChunkIter<'a, CHUNK_SIZE, BLOCK, METADATA> {
    type Item = (ChunkBlockPos, &'a Option<BLOCK>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x as usize == CHUNK_SIZE {
            return None;
        }

        let block_coord = ChunkBlockPos::new(self.x, self.y, self.z);
        let block = &self.chunk[&block_coord];

        self.z = (self.z + 1) % CHUNK_SIZE as u8;
        if self.z == 0 {
            self.y = (self.y + 1) % CHUNK_SIZE as u8;
            if self.y == 0 {
                self.x = self.x + 1;
            }
        }

        Some((block_coord, block))
    }
}