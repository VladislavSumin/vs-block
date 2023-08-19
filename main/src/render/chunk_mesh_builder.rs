use std::sync::RwLock;
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
            // Устанавливаем координаты блока в билдер (теперь добавленные меши будут автоматически
            // сдвинуты на эту величину
            builder.set_transition(block_pos.into());

            // Проходимся по всем граням блока
            for face_dir in AbsoluteBlockFaceDirection::iter() {
                // Если блок не имеет соседа со стороны проверяемой грани, то добавляем эту грань в меш
                if !is_need_to_render_face(chunk_map, chunk, chunk_pos, block_pos, face_dir) {
                    builder.add_mesh_data(face_dir);
                }
            }
        }
    }
    builder.build()
}

/// Возвращает нужно ли рендерить данную грань блока
fn is_need_to_render_face(
    chunk_map: &ChunkMap,
    chunk: &Chunk,
    chunk_pos: ChunkPos,
    block_pos: ChunkBlockPos,
    face_dir: AbsoluteBlockFaceDirection,
) -> bool {
    // Вычисляем глобальные координаты блока соседствующего с данным с переданной стороны
    let global_pos: AbsoluteBlockPos = chunk_pos + block_pos + face_dir;

    if let Ok(local_pos) = chunk_pos.try_global_pos_into_chunk_pos(global_pos) {
        // Соседний блок находится в текущем чанке
        chunk[&local_pos].is_some()
    } else {
        // Соседний блок находится в соседнем чанке
        let neighbor_chunk_pos = ChunkPos::from_global_coord(*global_pos);
        let chunk_map = chunk_map.read().unwrap();
        let neighbor_chunk = chunk_map.get(&neighbor_chunk_pos);
        match neighbor_chunk {
            None => {
                // Если соседний чанк не загружен то не рендерим грани блоков обращенных к нему
                true
            }
            Some(chunk) => {
                let block_pos = neighbor_chunk_pos.try_global_pos_into_chunk_pos(global_pos).unwrap();
                chunk.read().unwrap()[&block_pos].is_some()
            }
        }
    }
}