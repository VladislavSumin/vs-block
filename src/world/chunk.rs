use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;
pub const CHUNK_SQUARE: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u32 = CHUNK_SQUARE * CHUNK_SIZE;

/// Сущность описывающая один игровой чанк
pub struct Chunk {
    // temp solution
    pub blocks: [[[bool; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE],
}

impl Chunk {
    pub fn new(seed: u32) -> Self {
        let mut rand = StdRng::seed_from_u64(seed as u64);
        let mut blocks = [[[false; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE];

        for x in 0..CHUNK_SIZE_USIZE {
            for y in 0..CHUNK_SIZE_USIZE {
                for z in 0..CHUNK_SIZE_USIZE {
                    blocks[x][y][z] = rand.gen_bool(0.3)
                }
            }
        }

        Self {
            blocks
        }
    }
}