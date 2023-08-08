use bevy_math::{Vec3, vec3};
use generic_assert::{Assert, IsTrue};

/// Координаты блока в пределах чанка
/// TODO добавить compile time проверку выхода за границы допустимых для чанка индексов
#[derive(Default)]
pub struct ChunkBlockPos<const CHUNK_SIZE: usize> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl<const CHUNK_SIZE: usize> ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        assert!((x as usize) < CHUNK_SIZE);
        assert!((y as usize) < CHUNK_SIZE);
        assert!((z as usize) < CHUNK_SIZE);
        Self { x, y, z }
    }
}

impl<const CHUNK_SIZE: usize> Into<Vec3> for ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn into(self) -> Vec3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}
