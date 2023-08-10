use bevy::math::IVec3;
use bevy::prelude::Mesh;
use strum::IntoEnumIterator;
use chunk::{AbsoluteBlockPos, ChunkBlockPos, ChunkPos};
use crate::logic::chunk::Chunk;
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};

/// Строит [Mesh] для чанка
pub fn build_chunk_mesh(chunk: &Chunk, chunk_pos: ChunkPos) -> Mesh {
    let mut builder = MeshBuilder::new();
    for (block_coord, block) in chunk.into_iter() {
        if let Some(_) = block {
            builder.set_transition(block_coord.into());

            for dir in AbsoluteBlockFaceDirection::iter() {
                if !has_neighbor_block(chunk, chunk_pos, block_coord, dir) {
                    builder.add_mesh_data(dir);
                }
            }
        }
    }
    builder.build()
}

fn has_neighbor_block(chunk: &Chunk, chunk_pos: ChunkPos, pos: ChunkBlockPos, dir: AbsoluteBlockFaceDirection) -> bool {
    // Вычисляем глобальные координаты блока соседствующего с данным с переданной стороны
    let dir_pos: IVec3 = dir.into();
    let global_pos: AbsoluteBlockPos = (chunk_pos.get_absolute_coord() + pos.as_ivec3() + dir_pos).into();

    if let Ok(local_pos) = chunk_pos.try_global_pos_into_chunk_pos(global_pos) {
        chunk[&local_pos].is_some()
    } else {
        false
    }
}