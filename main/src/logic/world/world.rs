use std::collections::HashMap;
use bevy::log::info;
use bevy::prelude::{Entity, Resource};
use bevy::utils::HashSet;
use crate::logic::chunk::Chunk;
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
            let chunk = Chunk::new(entity, 32);
            self.chunks.insert(coord, chunk);
            entity
        } else {
            self.chunks.get(&coord).unwrap().get_entity()
        }
    }

    pub fn unload_chunk_if_exists(&mut self, coord: &ChunkCoord) -> Option<Entity> {
        self.chunks.remove(&coord).map(|chunk| { chunk.get_entity() })
    }
}
