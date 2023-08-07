use bevy::prelude::Resource;
use bevy::utils::HashMap;
use crate::logic::chunk::Chunk;
use crate::logic::world::ChunkCoord;

/// Структура мира
#[derive(Resource, Default)]
pub struct World {
    /// Список загруженных чанков
    chunks: HashMap<ChunkCoord, Chunk>,
}
