use crate::logic::block::Block;

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;

pub type Chunk = chunk::Chunk<CHUNK_SIZE_USIZE, Block, ()>;
pub type ChunkPos = chunk::ChunkPos<CHUNK_SIZE_USIZE>;
pub type ChunkMap = chunk::ChunkMap<CHUNK_SIZE_USIZE, Block, ()>;