use bevy::math::{Vec3, vec3};
use crate::logic::chunk::CHUNK_SIZE;

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не 16!
#[derive(Default)]
pub struct ChunkCoord {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl ChunkCoord {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> Vec3 {
        vec3(
            (self.x * CHUNK_SIZE as u32) as f32,
            (self.y * CHUNK_SIZE as u32) as f32,
            (self.z * CHUNK_SIZE as u32) as f32,
        )
    }
}
