use std::collections::HashMap;
use bevy::log::info;
use bevy::prelude::{Entity, Resource};
use bevy::utils::HashSet;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::logic::block::{Block, BlockType};
use crate::logic::chunk::{Chunk, CHUNK_SIZE_USIZE};
use crate::logic::world::ChunkCoord;

/// Структура мира
#[derive(Resource, Default)]
pub struct World {
    /// Список загруженных чанков
    chunks: HashMap<ChunkCoord, Chunk>,
}

impl World {
    /// Возвращает ссылку на чанк по [ChunkCoord] если такой чанк загружен в память
    pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    /// Возвращает список загруженных чанков
    pub fn get_chunk_keys(&self) -> HashSet<ChunkCoord> {
        self.chunks.iter().map(|(k, _)| { *k }).collect()
    }


    pub fn load_chunk_if_not_loaded<F>(&mut self, coord: ChunkCoord, entity_factory: F) -> Entity where
        F: FnOnce() -> Entity {
        if !self.chunks.contains_key(&coord) {
            info!("Loading chunk at {:?}", coord);
            let entity = entity_factory();
            let chunk = gen_chunk(entity, 32);
            self.chunks.insert(coord, chunk);
            entity
        } else {
            *self.chunks.get(&coord).unwrap().get_metadata()
        }
    }

    pub fn unload_chunk_if_exists(&mut self, coord: &ChunkCoord) -> Option<Entity> {
        self.chunks.remove(&coord).map(|chunk| { *chunk.get_metadata() })
    }
}

fn gen_chunk(entity: Entity, seed: i32) -> Chunk {
    let mut rand = StdRng::seed_from_u64(seed as u64);
    let mut blocks: [[[Option<Block>; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE]; CHUNK_SIZE_USIZE] = Default::default();

    for x in 0..CHUNK_SIZE_USIZE {
        for y in 0..CHUNK_SIZE_USIZE {
            for z in 0..CHUNK_SIZE_USIZE {
                if z == 0 {
                    blocks[x][y][z] = Some(Block::new(BlockType::BEDROCK));
                    continue;
                }
                let gen_block = rand.gen_bool(0.2);
                if gen_block {
                    blocks[x][y][z] = Some(Block::new(BlockType::GRASS));
                }
            }
        }
    }

    Chunk::new(blocks, entity)
}
