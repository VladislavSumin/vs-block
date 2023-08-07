use bevy::math::{Vec3, vec3};
use crate::logic::chunk::CHUNK_SIZE;

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не 16!
#[derive(Default)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Возвращает координаты чанка внутри которого находятся [coord]
    pub fn from_global_coord(coord: Vec3) -> Self {
        ChunkCoord::new(
            (coord.x as i32) / (CHUNK_SIZE as i32),
            (coord.y as i32) / (CHUNK_SIZE as i32),
            (coord.z as i32) / (CHUNK_SIZE as i32),
        )
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> Vec3 {
        vec3(
            (self.x * CHUNK_SIZE as i32) as f32,
            (self.y * CHUNK_SIZE as i32) as f32,
            (self.z * CHUNK_SIZE as i32) as f32,
        )
    }
}
