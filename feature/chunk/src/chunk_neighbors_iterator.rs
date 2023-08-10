use std::ops::Deref;
use bevy_math::{IVec3, ivec3};
use crate::ChunkPos;

pub struct ChunkNeighborsIterator {
    center: ChunkPos,
    index: u32,
}

impl ChunkNeighborsIterator {
    pub fn new(center: ChunkPos) -> Self {
        Self {
            center,
            index: 0,
        }
    }
    fn get_delta(&self, delta: IVec3) -> ChunkPos {
        (*self.center.deref() + delta).into()
    }
}

impl Iterator for ChunkNeighborsIterator {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => { Some(self.get_delta(ivec3(1, 0, 0))) }
            1 => { Some(self.get_delta(ivec3(-1, 0, 0))) }
            2 => { Some(self.get_delta(ivec3(0, 1, 0))) }
            3 => { Some(self.get_delta(ivec3(0, -1, 0))) }
            4 => { Some(self.get_delta(ivec3(0, 0, 1))) }
            5 => { Some(self.get_delta(ivec3(0, 0, -1))) }
            _ => { None }
        }
    }
}