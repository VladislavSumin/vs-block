use bevy_derive::Deref;
use bevy_math::{IVec3, UVec3, Vec3};
use crate::chunk::CHUNK_SIZE;

/// Координаты блока в пределах чанка
/// TODO добавить compile time проверку выхода за границы допустимых для чанка индексов
#[derive(Default, Copy, Clone, Deref)]
pub struct ChunkBlockPos {
    pos: UVec3,
}

impl TryFrom<UVec3> for ChunkBlockPos {
    type Error = ();

    fn try_from(value: UVec3) -> Result<Self, Self::Error> {
        if (value.x as usize) < CHUNK_SIZE && (value.y as usize) < CHUNK_SIZE && (value.z as usize) < CHUNK_SIZE {
            Ok(ChunkBlockPos { pos: value })
        } else {
            Err(())
        }
    }
}

impl TryFrom<IVec3> for ChunkBlockPos {
    type Error = ();

    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        if value.x >= 0 && value.y >= 0 && value.z >= 0 &&
            (value.x as usize) < CHUNK_SIZE && (value.y as usize) < CHUNK_SIZE && (value.z as usize) < CHUNK_SIZE {
            Ok(ChunkBlockPos { pos: value.as_uvec3() })
        } else {
            Err(())
        }
    }
}

impl Into<Vec3> for ChunkBlockPos {
    fn into(self) -> Vec3 {
        self.as_vec3()
    }
}
