use std::collections::HashMap;
use bevy::log::info;
use bevy::prelude::{Entity, Resource};
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
    pub fn get_chunks(&self) -> &HashMap<ChunkCoord, Chunk> {
        &self.chunks
    }


    pub fn load_chunk_if_not_loaded<F>(&mut self, coord: ChunkCoord, entity_factory: F) where
        F: FnOnce() -> Entity {
        if !self.chunks.contains_key(&coord) {
            info!("Loading chunk at {:?}", coord);
            let chunk = Chunk::new(entity_factory(), 32);
            self.chunks.insert(coord, chunk);
        }
    }
}
