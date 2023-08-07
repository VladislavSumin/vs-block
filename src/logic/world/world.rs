use std::collections::HashMap;
use bevy::log::info;
use bevy::prelude::{Entity, Resource};
use crate::logic::chunk::Chunk;
use crate::logic::world::ChunkCoord;

/// Структура мира
#[derive(Resource, Default)]
pub struct World {
    /// Список загруженных чанков
    chunks: HashMap<ChunkCoord, ChunkData>,
}

struct ChunkData {
    chunk: Chunk,
    entity: Entity,
}

impl World {
    pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord).map(|chunk_data| { &chunk_data.chunk })
    }

    pub fn load_chunk_if_not_loaded<F>(&mut self, coord: ChunkCoord, entity_factory: F) where
        F: FnOnce() -> Entity {
        if !self.chunks.contains_key(&coord) {
            info!("Loading chunk at {:?}", coord);
            let chunk = Chunk::new(32);
            let chunk_data = ChunkData {
                chunk,
                entity: entity_factory(),
            };
            self.chunks.insert(coord, chunk_data);
        }
    }
}
