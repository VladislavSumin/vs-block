use bevy::math::ivec3;
use bevy::prelude::*;
use bevy::utils::HashSet;
use chunk::ChunkPos;
use world_anchor::{WorldAnchor, WorldAnchorInChunkPos};
use crate::logic::world::world::World;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<World>()
            .init_resource::<ChunkLoadingQueue>()
            .add_event::<ChunkUpdateEvent>()
            .add_systems(Update, manage_chunk_loading_state)
            .add_systems(Update, load_new_chunks_from_queue)
        ;
    }
}

#[derive(Event)]
pub enum ChunkUpdateEvent {
    Loaded(ChunkPos),
    Unloaded(ChunkPos),
}

#[derive(Resource, Default)]
struct ChunkLoadingQueue {
    pub positions: HashSet<ChunkPos>,
}

/// Загружает управлаяет очередью загрузки чанков, а так же выгружает не нужные чанки из памяти
fn manage_chunk_loading_state(
    mut world: ResMut<World>,
    mut chunk_loading_queue: ResMut<ChunkLoadingQueue>,
    mut chunk_event_writer: EventWriter<ChunkUpdateEvent>,
    changed_world_anchors_pos: Query<(), Changed<WorldAnchorInChunkPos>>,
    changed_world_anchors_conf: Query<(), Changed<WorldAnchor>>,
    world_anchors_pos: Query<(&WorldAnchorInChunkPos, &WorldAnchor)>,
) {
    // Если позиции и параметры якорей не изменились то пересчитывать не нужно
    if changed_world_anchors_pos.is_empty() && changed_world_anchors_conf.is_empty() {
        return;
    }
    info!("manage_chunk_loading_state()");

    // Список чанков которые нужно удалить
    let mut chunks_to_unload = world.get_chunk_keys();

    // Список чанков которые нужно загрузить
    let mut chunks_to_load = HashSet::<ChunkPos>::new();

    // Итерируемся по всем WorldAnchor
    for (pos, world_anchor) in world_anchors_pos.iter() {
        let load_radius = world_anchor.load_radius as i32;

        // Получаем координаты чанка в котором находится WorldAnchor
        let mut anchor_chunk_coord = pos.pos;
        anchor_chunk_coord.z = 0;

        for x in anchor_chunk_coord.x - load_radius..anchor_chunk_coord.x + load_radius {
            for y in anchor_chunk_coord.y - load_radius..anchor_chunk_coord.y + load_radius {
                for z in 0..(512 / 16) {
                    let pos = ivec3(x, y, z).into();
                    // Удаляем чанк находящийся внутри радиуса из списка чанков на удаление
                    if !chunks_to_unload.remove(&pos) {
                        // Если такого чанка вообще не было среди загруженных, добавляем его в очередь на загрузку
                        if !world.is_chunk_loaded(&pos) {
                            chunks_to_load.insert(pos);
                        }
                    }
                }
            }
        }
    }

    // Удаляем старые чанки
    for chunk_coord in chunks_to_unload {
        world.remove_chunk(&chunk_coord);
        chunk_event_writer.send(ChunkUpdateEvent::Unloaded(chunk_coord))
    }

    // Обновляем очередь на загрузку чанков
    chunk_loading_queue.positions = chunks_to_load;
}

/// Загружает чинки из очереди [ChunkLoadingQueue]
fn load_new_chunks_from_queue(
    mut world: ResMut<World>,
    mut chunk_event_writer: EventWriter<ChunkUpdateEvent>,
    mut chunk_loading_queue: ResMut<ChunkLoadingQueue>,
) {
    let mut generated_chunks: HashSet<ChunkPos> = HashSet::new();
    for pos in chunk_loading_queue.positions.iter().take(100) {
        world.add_chunk(*pos);
        chunk_event_writer.send(ChunkUpdateEvent::Loaded(*pos));
        generated_chunks.insert(*pos);
    }
    for pos in generated_chunks {
        chunk_loading_queue.positions.remove(&pos);
    }
}
