use std::ops::Deref;
use bevy_math::{IVec3, ivec3, Vec3};
use generic_assert::{Assert, IsTrue};

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не CHUNK_SIZE!
/// [CHUNK_SIZE] Размер чанка, в данном случае нужен для вычисления координат чанка из глобальных координат
#[derive(Default, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ChunkPos<const CHUNK_SIZE: usize> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pos: IVec3,
}

impl<const CHUNK_SIZE: usize> ChunkPos<CHUNK_SIZE> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    /// Возвращает координаты чанка внутри которого находятся [coord]
    pub fn from_global_coord(coord: Vec3) -> Self {
        let coord = coord.as_ivec3();
        ivec3(
            (coord.x + (coord.x < 0) as i32) / CHUNK_SIZE as i32 - (coord.x < 0) as i32,
            (coord.y + (coord.y < 0) as i32) / CHUNK_SIZE as i32 - (coord.y < 0) as i32,
            (coord.z + (coord.z < 0) as i32) / CHUNK_SIZE as i32 - (coord.z < 0) as i32,
        ).into()
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> Vec3 {
        (self.pos * CHUNK_SIZE as i32).as_vec3()
    }

    pub fn raw_pos(&self) -> &IVec3 {
        &self.pos
    }

    pub fn set_z(&mut self, z: i32) {
        self.pos.z = z;
    }
}

impl<const CHUNK_SIZE: usize> Deref for ChunkPos<CHUNK_SIZE> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl<const CHUNK_SIZE: usize> From<IVec3> for ChunkPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn from(value: IVec3) -> Self {
        ChunkPos { pos: value }
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::vec3;
    use crate::chunk_pos::ChunkPos as ChunkCoordTemplate;

    type ChunkCoord = ChunkCoordTemplate<16>;

    #[test]
    fn from_global_coord_min_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(0., 0., 0.));
        assert_eq!(coord.pos.x, 0);
        assert_eq!(coord.pos.y, 0);
        assert_eq!(coord.pos.z, 0);
    }

    #[test]
    fn from_global_coord_inside_chunk_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(1., 2., 15.99));
        assert_eq!(coord.pos.x, 0);
        assert_eq!(coord.pos.y, 0);
        assert_eq!(coord.pos.z, 0);
    }

    #[test]
    fn from_global_coord_second_chunk() {
        let coord = ChunkCoord::from_global_coord(vec3(18., 19., 22.));
        assert_eq!(coord.pos.x, 1);
        assert_eq!(coord.pos.y, 1);
        assert_eq!(coord.pos.z, 1);
    }

    #[test]
    fn from_global_coord_negative_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(-1., -2., 2.));
        assert_eq!(coord.pos.x, -1);
        assert_eq!(coord.pos.y, -1);
        assert_eq!(coord.pos.z, 0);
    }
}