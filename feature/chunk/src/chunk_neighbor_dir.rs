use std::ops::{Add, Deref};
use bevy_math::IVec3;
use strum_macros::EnumIter;
use crate::ChunkPos;

#[derive(EnumIter, Copy, Clone)]
pub enum ChunkNeighborDir {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

/// Сумма [ChunkPos] + [ChunkNeighborDir] = [ChunkPos]
impl Add<ChunkNeighborDir> for ChunkPos {
    type Output = ChunkPos;

    fn add(self, rhs: ChunkNeighborDir) -> Self::Output {
        let vec = match rhs {
            ChunkNeighborDir::PosX => { IVec3::X }
            ChunkNeighborDir::NegX => { IVec3::NEG_X }
            ChunkNeighborDir::PosY => { IVec3::Y }
            ChunkNeighborDir::NegY => { IVec3::NEG_Y }
            ChunkNeighborDir::PosZ => { IVec3::Z }
            ChunkNeighborDir::NegZ => { IVec3::NEG_Z }
        };
        (*(self.deref()) + vec).into()
    }
}
