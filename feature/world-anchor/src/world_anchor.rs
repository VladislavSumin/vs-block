use bevy::prelude::Component;
use chunk::ChunkPos;
use generic_assert::{Assert, IsTrue};

const DEFAULT_LOAD_RADIUS: u32 = 2;

/// Маркерный интерфейс, для маркировки сущностей вокруг которых должен грузиться мир
/// Такие сущности обязательно должны так же включать элемент [Transform]
#[derive(Component)]
pub struct WorldAnchor {
    /// Радиус в пределах которого будет загружаться мир
    pub load_radius: u32,
}

impl Default for WorldAnchor {
    fn default() -> Self {
        WorldAnchor {
            load_radius: DEFAULT_LOAD_RADIUS
        }
    }
}

/// Позиция [WorldAnchor] в сетке чанков
#[derive(Component)]
pub struct WorldAnchorInChunkPos<const CHUNK_SIZE: usize>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pub pos: ChunkPos<CHUNK_SIZE>,
}
