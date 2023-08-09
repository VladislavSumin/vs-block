use std::collections::HashMap;
use bevy::log::info;
use bevy::prelude::Resource;
use bevy::utils::HashSet;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use chunk::ChunkBlockPos;
use crate::logic::block::{Block, BlockType};
use crate::logic::chunk::{Chunk, CHUNK_SIZE_USIZE, ChunkPos};

/// Структура мира
#[derive(Resource, Default)]
pub struct World {
    /// Список загруженных чанков
    chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    /// Возвращает загружен ли чанк по переданной позиции
    pub fn is_chunk_loaded(&self, pos: &ChunkPos) -> bool {
        self.chunks.contains_key(pos)
    }

    /// Возвращает ссылку на чанк по [ChunkCoord] если такой чанк загружен в память
    pub fn get_chunk(&self, coord: &ChunkPos) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    /// Возвращает список загруженных чанков
    pub fn get_chunk_keys(&self) -> HashSet<ChunkPos> {
        self.chunks.iter().map(|(k, _)| { *k }).collect()
    }

    /// Добавляет новый чанк, если чанк по этим координатам уже загружен паникует
    pub fn add_chunk(&mut self, coord: ChunkPos) {
        assert!(!self.chunks.contains_key(&coord));
        let chunk = gen_chunk(32);
        self.chunks.insert(coord, chunk);
    }

    /// Удаляет чанк, если чанк по этим координатам уже удален паникует
    pub fn remove_chunk(&mut self, coord: &ChunkPos) {
        self.chunks.remove(&coord).unwrap();
    }
}

fn gen_chunk(seed: i32) -> Chunk {
    let mut rand = StdRng::seed_from_u64(seed as u64);
    let mut chunk = Chunk::new(());

    for x in 0..CHUNK_SIZE_USIZE {
        for y in 0..CHUNK_SIZE_USIZE {
            for z in 0..CHUNK_SIZE_USIZE {
                let pos = ChunkBlockPos::new(x as u8, y as u8, z as u8);
                if z == 0 {
                    chunk[&pos] = Some(Block::new(BlockType::BEDROCK));
                    continue;
                }
                let gen_block = rand.gen_bool(0.2);
                if gen_block {
                    chunk[&pos] = Some(Block::new(BlockType::GRASS));
                }
            }
        }
    }
    chunk
}
