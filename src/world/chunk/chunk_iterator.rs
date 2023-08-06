use crate::world::block::Block;
use crate::world::chunk::{Chunk, CHUNK_SIZE, ChunkBlockCoord};

impl<'a> IntoIterator for &'a Chunk {
    type Item = (ChunkBlockCoord, &'a Option<Block>);
    type IntoIter = ChunkIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {
            chunk: self,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    x: u8,
    y: u8,
    z: u8,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = (ChunkBlockCoord, &'a Option<Block>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == CHUNK_SIZE {
            return None;
        }

        let block_coord = ChunkBlockCoord::new(self.x, self.y, self.z);
        let block = &self.chunk[&block_coord];

        self.z = (self.z + 1) % CHUNK_SIZE;
        if self.z == 0 {
            self.y = (self.y + 1) % CHUNK_SIZE;
            if self.y == 0 {
                self.x = self.x + 1;
            }
        }

        Some((block_coord, block))
    }
}