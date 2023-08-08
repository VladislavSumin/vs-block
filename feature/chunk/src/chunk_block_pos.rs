use bevy_math::{Vec3, vec3};

/// Координаты блока в пределах чанка
/// TODO добавить compile time проверку выхода за границы допустимых для чанка индексов
#[derive(Default)]
pub struct ChunkBlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl ChunkBlockPos {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        // TODO добавить проверки на выход за границы допустимых значений
        Self { x, y, z }
    }
}

impl Into<Vec3> for ChunkBlockPos {
    fn into(self) -> Vec3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}
