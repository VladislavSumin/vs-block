use std::cmp::min;
use std::sync::{Arc, RwLock};
use bevy::math::uvec3;
use bevy::prelude::Resource;
use bevy::utils::HashSet;
use noise::NoiseFn;
use chunk::{CHUNK_SIZE, ChunkPos};
use crate::logic::block::{Block, BlockType};
use crate::logic::chunk::{Chunk, ChunkMap};

pub const WORLD_HEIGHT: usize = 256;
pub const WORLD_HEIGHT_CHUNKS: usize = WORLD_HEIGHT / CHUNK_SIZE;

type Noise = noise::Fbm<noise::SuperSimplex>;

/// Структура мира
#[derive(Resource)]
pub struct World {
    pub noise: Noise,

    /// Список загруженных чанков
    pub chunk_map: ChunkMap,
}

impl Default for World {
    fn default() -> Self {
        let mut noise = noise::Fbm::new(3);
        noise.frequency = 0.05;
        noise.persistence = 0.25;
        World {
            noise,
            chunk_map: ChunkMap::default(),
        }
    }
}

impl World {
    /// Возвращает загружен ли чанк по переданной позиции
    pub fn is_chunk_loaded(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.read().unwrap().contains_key(pos)
    }

    /// Возвращает ссылку на чанк по [ChunkCoord] если такой чанк загружен в память
    pub fn get_chunk(&self, coord: &ChunkPos) -> Option<Arc<RwLock<Chunk>>> {
        self.chunk_map.read().unwrap().get(coord).map(|arc| Arc::clone(arc))
    }

    /// Возвращает список загруженных чанков
    pub fn get_chunk_keys(&self) -> HashSet<ChunkPos> {
        self.chunk_map.read().unwrap().iter().map(|(k, _)| { *k }).collect()
    }

    /// Добавляет новый чанк, если чанк по этим координатам уже загружен паникует
    pub fn add_chunk(&self, pos: ChunkPos, chunk: Chunk) {
        let mut chunk_map = self.chunk_map.write().unwrap();
        assert!(!chunk_map.contains_key(&pos));
        let chunk = Arc::new(RwLock::new(chunk));
        chunk_map.insert(pos, chunk);
    }

    /// Удаляет чанк, если чанк по этим координатам уже удален паникует
    pub fn remove_chunk(&self, coord: &ChunkPos) {
        self.chunk_map.write().unwrap().remove(&coord).unwrap();
    }
}

pub fn gen_chunk(pos: ChunkPos, noise: &Noise) -> Chunk {
    let mut chunk = Chunk::new(());

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let h = noise.get([(pos.x * CHUNK_SIZE as i32 + x as i32) as f64 * 0.14, (pos.y * CHUNK_SIZE as i32 + y as i32) as f64 * 0.14]);
            let h = (h + 1.0) / 2.0;
            let h = h / 2.0 + 0.2;

            let h = (128f64 * h) as i32;
            let h = h - (pos.z * CHUNK_SIZE as i32);
            if h <= 0 { continue; }

            for z in 0..min(h as usize, CHUNK_SIZE) {
                let pos = uvec3(x as u32, y as u32, z as u32).try_into().unwrap();
                chunk[&pos] = Some(Block::new(BlockType::GRASS));
            }
        }
    }
    chunk
}
