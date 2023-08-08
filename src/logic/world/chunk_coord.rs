use bevy::math::{IVec3, ivec3, Vec3};
use bevy::prelude::Component;
use crate::logic::chunk::CHUNK_SIZE;

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не 16!
#[derive(Default, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ChunkCoord {
    coord: IVec3,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { coord: ivec3(x, y, z) }
    }

    /// Возвращает координаты чанка внутри которого находятся [coord]
    pub fn from_global_coord(coord: Vec3) -> Self {
        let coord = coord.as_ivec3();
        ChunkCoord::new(
            (coord.x + (coord.x < 0) as i32) / CHUNK_SIZE as i32 - (coord.x < 0) as i32,
            (coord.y + (coord.y < 0) as i32) / CHUNK_SIZE as i32 - (coord.y < 0) as i32,
            (coord.z + (coord.z < 0) as i32) / CHUNK_SIZE as i32 - (coord.z < 0) as i32,
        )
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> Vec3 {
        (self.coord * CHUNK_SIZE as i32).as_vec3()
    }

    pub fn raw_pos(&self) -> &IVec3 {
        &self.coord
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::vec3;
    use crate::logic::world::ChunkCoord;

    #[test]
    fn from_global_coord_min_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(0., 0., 0.));
        assert_eq!(coord.coord.x, 0);
        assert_eq!(coord.coord.y, 0);
        assert_eq!(coord.coord.z, 0);
    }

    #[test]
    fn from_global_coord_inside_chunk_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(1., 2., 15.99));
        assert_eq!(coord.coord.x, 0);
        assert_eq!(coord.coord.y, 0);
        assert_eq!(coord.coord.z, 0);
    }

    #[test]
    fn from_global_coord_second_chunk() {
        let coord = ChunkCoord::from_global_coord(vec3(18., 19., 22.));
        assert_eq!(coord.coord.x, 1);
        assert_eq!(coord.coord.y, 1);
        assert_eq!(coord.coord.z, 1);
    }

    #[test]
    fn from_global_coord_negative_coord() {
        let coord = ChunkCoord::from_global_coord(vec3(-1., -2., 2.));
        assert_eq!(coord.coord.x, -1);
        assert_eq!(coord.coord.y, -1);
        assert_eq!(coord.coord.z, 0);
    }
}