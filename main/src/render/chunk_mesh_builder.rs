use bevy::prelude::Mesh;
use strum::IntoEnumIterator;
use chunk::{AbsoluteBlockPos, ChunkBlockPos, ChunkPos};
use crate::logic::chunk::{Chunk, ChunkMap};
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};

/// Строит [Mesh] для чанка
pub fn build_chunk_mesh(chunk_map: &ChunkMap, chunk: &Chunk, chunk_pos: ChunkPos) -> Mesh {
    let mut builder = MeshBuilder::new();

    // Итерируемся по всем блокам
    for (block_pos, block) in chunk.into_iter() {
        if let Some(_) = block {
            // Устанавливаем координаты блока в блидер (теперь добавленные меши будут автоматически
            // сдвинуты на эту величину
            builder.set_transition(block_pos.into());

            // Проходимся по всем граням блока
            for face_dir in AbsoluteBlockFaceDirection::iter() {
                // Если блок не имеет соседа со стороны проверяемой грани, то добавляем эту грань в меш
                if !has_neighbor_block(chunk_map, chunk, chunk_pos, block_pos, face_dir) {
                    builder.add_mesh_data(face_dir);
                }
            }
        }
    }
    builder.build()
}

/// Возвращает имеется ли блок со стороны переданной грани блока
fn has_neighbor_block(
    chunk_map: &ChunkMap,
    chunk: &Chunk,
    chunk_pos: ChunkPos,
    block_pos: ChunkBlockPos,
    face_dir: AbsoluteBlockFaceDirection,
) -> bool {
    // Если это самый нижний блок (дно мира) то снизу у него "есть" соседи, это нужно для отсечения нижних граней
    // потому что снизу на мир никто смотреть не должен
    if chunk_pos.z == 0 && block_pos.z == 0 { return true; }

    // Вычисляем глобальные координаты блока соседствующего с данным с переданной стороны
    let global_pos: AbsoluteBlockPos = chunk_pos + block_pos + face_dir;

    if let Ok(local_pos) = chunk_pos.try_global_pos_into_chunk_pos(global_pos) {
        chunk[&local_pos].is_some()
    } else {
        chunk_map.get_block_at(global_pos).is_some()
    }
}