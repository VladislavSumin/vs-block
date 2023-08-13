use std::ops::Add;
use bevy_derive::{Deref, DerefMut};
use bevy_math::{IVec3, ivec3};
use crate::absolute_block_pos::AbsoluteBlockPos;
use crate::chunk::CHUNK_SIZE;
use crate::chunk_neighbors_iter::ChunkNeighborsIter;
use crate::ChunkBlockPos;

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не CHUNK_SIZE!
#[derive(Default, Hash, Eq, PartialEq, Clone, Copy, Debug, Deref, DerefMut)]
pub struct ChunkPos {
    pos: IVec3,
}

impl ChunkPos {
    /// Возвращает координаты чанка внутри которого находятся [coord]
    pub fn from_global_coord(coord: IVec3) -> Self {
        ivec3(
            (coord.x + (coord.x < 0) as i32) / CHUNK_SIZE as i32 - (coord.x < 0) as i32,
            (coord.y + (coord.y < 0) as i32) / CHUNK_SIZE as i32 - (coord.y < 0) as i32,
            (coord.z + (coord.z < 0) as i32) / CHUNK_SIZE as i32 - (coord.z < 0) as i32,
        ).into()
    }

    /// Проверяет принадлежат ли глобальные координаты блока текущему чанку, и если принадлежат возвращают локальные
    /// координаты в этом чанке
    pub fn try_global_pos_into_chunk_pos(&self, global_pos: AbsoluteBlockPos) -> Result<ChunkBlockPos, ()> {
        // Вычитаем из глобальных координат координаты текущего чанка в абсолютной системе координат
        let local_pos = *global_pos - self.get_absolute_coord();
        // Пробуем привести полученное значение к координатам чанка (если переданный блок не лежит в этом чанке у
        // у нас ничего из этого не выйдет)
        local_pos.try_into()
    }

    /// Возвращает итератор из координат чанков окружающих этот (6 штук)
    pub fn get_neighbors(&self) -> ChunkNeighborsIter {
        ChunkNeighborsIter::new(*self)
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> IVec3 {
        self.pos * CHUNK_SIZE as i32
    }
}

impl From<IVec3> for ChunkPos {
    fn from(value: IVec3) -> Self {
        ChunkPos { pos: value }
    }
}

/// Сумма [ChunkPos] + [ChunkBlockPos] = [AbsoluteBlockPos]
impl Add<ChunkBlockPos> for ChunkPos {
    type Output = AbsoluteBlockPos;

    fn add(self, rhs: ChunkBlockPos) -> Self::Output {
        (self.get_absolute_coord() + rhs.as_ivec3()).into()
    }
}

/// [ChunkPos] из [AbsoluteBlockPos]
/// Позиция чанка в котором находится блок
impl From<AbsoluteBlockPos> for ChunkPos {
    fn from(value: AbsoluteBlockPos) -> Self {
        ChunkPos::from_global_coord(value.into())
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::{ivec3, vec3};
    use crate::chunk_pos::ChunkPos;


    #[test]
    fn from_global_coord_min_coord() {
        let pos = ChunkPos::from_global_coord(ivec3(0, 0, 0));
        assert_eq!(pos.pos.x, 0);
        assert_eq!(pos.pos.y, 0);
        assert_eq!(pos.pos.z, 0);
    }

    #[test]
    fn from_global_coord_inside_chunk_coord() {
        let pos = ChunkPos::from_global_coord(ivec3(1, 2, 15));
        assert_eq!(pos.pos.x, 0);
        assert_eq!(pos.pos.y, 0);
        assert_eq!(pos.pos.z, 0);
    }

    #[test]
    fn from_global_coord_second_chunk() {
        let pos = ChunkPos::from_global_coord(ivec3(18, 19, 22));
        assert_eq!(pos.pos.x, 1);
        assert_eq!(pos.pos.y, 1);
        assert_eq!(pos.pos.z, 1);
    }

    #[test]
    fn from_global_coord_negative_coord() {
        let pos = ChunkPos::from_global_coord(ivec3(-1, -2, 2));
        assert_eq!(pos.pos.x, -1);
        assert_eq!(pos.pos.y, -1);
        assert_eq!(pos.pos.z, 0);
    }
}