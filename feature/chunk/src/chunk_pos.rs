use std::ops::{Add, Deref, DerefMut};
use bevy_math::{IVec3, ivec3};
use generic_assert::{Assert, IsTrue};
use crate::absolute_block_pos::AbsoluteBlockPos;
use crate::ChunkBlockPos;

/// Координаты чанка в сетке чанков.
/// **Note** Например, второй чанк по оси Х, будет иметь координату X = 1, а не CHUNK_SIZE!
/// [CHUNK_SIZE] Размер чанка, в данном случае нужен для вычисления координат чанка из глобальных координат
#[derive(Default, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ChunkPos<const CHUNK_SIZE: usize> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    pos: IVec3,
}

impl<const CHUNK_SIZE: usize> ChunkPos<CHUNK_SIZE> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
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
    fn try_global_pos_into_chunk_pos(&self, global_pos: AbsoluteBlockPos) -> Result<ChunkBlockPos<CHUNK_SIZE>, ()> {
        // Вычитаем из глобальных координат координаты текущего чанка в абсолютной системе координат
        let local_pos = *global_pos - self.get_absolute_coord();
        // Пробуем привести полученное значение к координатам чанка (если переданный блок не лежит в этом чанке у
        // у нас ничего из этого не выйдет)
        local_pos.try_into()
    }

    /// Возвращает абсолютные координаты чанка в мире
    pub fn get_absolute_coord(&self) -> IVec3 {
        self.pos * CHUNK_SIZE as i32
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

impl<const CHUNK_SIZE: usize> DerefMut for ChunkPos<CHUNK_SIZE> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
}

impl<const CHUNK_SIZE: usize> From<IVec3> for ChunkPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn from(value: IVec3) -> Self {
        ChunkPos { pos: value }
    }
}

/// Сумма [ChunkPos] + [ChunkBlockPos] = [AbsoluteBlockPos]
impl<const CHUNK_SIZE: usize> Add<ChunkBlockPos<CHUNK_SIZE>> for ChunkPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    type Output = AbsoluteBlockPos;

    fn add(self, rhs: ChunkBlockPos<CHUNK_SIZE>) -> Self::Output {
        (self.get_absolute_coord() + rhs.as_ivec3()).into()
    }
}

/// [ChunkPos] из [AbsoluteBlockPos]
/// Позиция чанка в котором находится блок
impl<const CHUNK_SIZE: usize> From<AbsoluteBlockPos> for ChunkPos<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn from(value: AbsoluteBlockPos) -> Self {
        ChunkPos::from_global_coord(value.into())
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::{ivec3, vec3};
    use crate::chunk_pos::ChunkPos as ChunkCoordTemplate;

    type ChunkCoord = ChunkCoordTemplate<16>;

    #[test]
    fn from_global_coord_min_coord() {
        let coord = ChunkCoord::from_global_coord(ivec3(0, 0, 0));
        assert_eq!(coord.pos.x, 0);
        assert_eq!(coord.pos.y, 0);
        assert_eq!(coord.pos.z, 0);
    }

    #[test]
    fn from_global_coord_inside_chunk_coord() {
        let coord = ChunkCoord::from_global_coord(ivec3(1, 2, 15));
        assert_eq!(coord.pos.x, 0);
        assert_eq!(coord.pos.y, 0);
        assert_eq!(coord.pos.z, 0);
    }

    #[test]
    fn from_global_coord_second_chunk() {
        let coord = ChunkCoord::from_global_coord(ivec3(18, 19, 22));
        assert_eq!(coord.pos.x, 1);
        assert_eq!(coord.pos.y, 1);
        assert_eq!(coord.pos.z, 1);
    }

    #[test]
    fn from_global_coord_negative_coord() {
        let coord = ChunkCoord::from_global_coord(ivec3(-1, -2, 2));
        assert_eq!(coord.pos.x, -1);
        assert_eq!(coord.pos.y, -1);
        assert_eq!(coord.pos.z, 0);
    }
}