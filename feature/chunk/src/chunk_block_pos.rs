use std::ops::Deref;
use bevy_math::{IVec3, UVec3, Vec3, vec3};
use generic_assert::{Assert, IsTrue};

/// Координаты блока в пределах чанка
/// TODO добавить compile time проверку выхода за границы допустимых для чанка индексов
#[derive(Default, Copy, Clone)]
pub struct ChunkBlockPos<const CHUNK_SIZE: usize> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pos: UVec3,
}

impl<const CHUNK_SIZE: usize> ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {}

impl<const CHUNK_SIZE: usize> TryFrom<UVec3> for ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Error = ();

    fn try_from(value: UVec3) -> Result<Self, Self::Error> {
        if (value.x as usize) < CHUNK_SIZE && (value.y as usize) < CHUNK_SIZE && (value.z as usize) < CHUNK_SIZE {
            Ok(ChunkBlockPos { pos: value })
        } else {
            Err(())
        }
    }
}

impl<const CHUNK_SIZE: usize> TryFrom<IVec3> for ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
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

impl<const CHUNK_SIZE: usize> Deref for ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Target = UVec3;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl<const CHUNK_SIZE: usize> Into<Vec3> for ChunkBlockPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn into(self) -> Vec3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}
