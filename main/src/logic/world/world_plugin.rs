use std::cmp::min;
use bevy::math::ivec3;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::utils::HashMap;
use futures_lite::future::{block_on, poll_once};
use chunk::ChunkPos;
use world_anchor::{WorldAnchor, WorldAnchorInChunkPos};
use crate::logic::chunk::Chunk;
use crate::logic::world::world::{gen_chunk, World, WORLD_HEIGHT_CHUNKS};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<World>()
            .init_resource::<ChunkLoadingQueue>()
            .init_resource::<ChunkLoadingTasks>()
            .add_event::<ChunkUpdateEvent>()
            .add_systems(Update, manage_chunk_loading_state)
            .add_systems(Update, load_new_chunks_from_queue)
            .add_systems(Update, spawn_loaded_chunks)
        ;
    }
}

#[derive(Event)]
pub enum ChunkUpdateEvent {
    Loaded(ChunkPos),
    Unloaded(ChunkPos),
}

#[derive(Resource, Default, Deref, DerefMut)]
struct ChunkLoadingQueue(Vec<ChunkLoadInfo>);

#[derive(Resource, Default, Deref, DerefMut)]
struct ChunkLoadingTasks(HashMap<ChunkPos, Task<Chunk>>);

#[derive(Copy, Clone)]
struct ChunkLoadInfo {
    pos: ChunkPos,
    priority: i32,
}


/// Загружает управлаяет очередью загрузки чанков, а так же выгружает не нужные чанки из памяти
fn manage_chunk_loading_state(
    world: Res<World>,
    mut chunk_loading_queue: ResMut<ChunkLoadingQueue>,
    mut chunk_event_writer: EventWriter<ChunkUpdateEvent>,
    chunk_loading_tasks: ResMut<ChunkLoadingTasks>,
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
    let mut chunks_to_load = HashMap::<ChunkPos, ChunkLoadInfo>::new();

    // Итерируемся по всем WorldAnchor
    for (pos, world_anchor) in world_anchors_pos.iter() {
        let load_radius = world_anchor.load_radius as i32;

        // Получаем координаты чанка в котором находится WorldAnchor
        let mut anchor_chunk_coord = pos.pos;
        anchor_chunk_coord.z = 0;

        if load_radius == 0 { continue; }

        for x in anchor_chunk_coord.x - load_radius + 1..anchor_chunk_coord.x + load_radius {
            for y in anchor_chunk_coord.y - load_radius + 1..anchor_chunk_coord.y + load_radius {
                for z in 0..(WORLD_HEIGHT_CHUNKS as i32) {
                    let pos = ivec3(x, y, z).into();
                    // Удаляем чанк находящийся внутри радиуса из списка чанков на удаление
                    if !chunks_to_unload.remove(&pos) {
                        // Если такого чанка вообще не было среди загруженных, добавляем его в очередь на загрузку
                        if !world.is_chunk_loaded(&pos) && !chunk_loading_tasks.contains_key(&pos) {
                            let chunk_load_info = ChunkLoadInfo {
                                pos,
                                priority: anchor_chunk_coord.distance_squared(*pos),
                            };
                            // TODO если тут уже есть значение нужно выставить правильный приоритет
                            chunks_to_load.insert(pos, chunk_load_info);
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

    chunk_loading_queue.clear();
    chunk_loading_queue.extend(chunks_to_load.iter().map(|pos| pos.1));
    chunk_loading_queue.sort_by(|a, b| { a.priority.cmp(&b.priority) })
}

/// Загружает чинки из очереди [ChunkLoadingQueue]
fn load_new_chunks_from_queue(
    world: Res<World>,
    mut chunk_loading_queue: ResMut<ChunkLoadingQueue>,
    mut chunk_loading_tasks: ResMut<ChunkLoadingTasks>,
) {
    let pool = AsyncComputeTaskPool::get();
    let size = min(64 - chunk_loading_tasks.len(), chunk_loading_queue.len());
    let new_tasks = chunk_loading_queue.drain(0..size)
        .map(|chunk_loading_info| {
            let noise = world.noise.clone();
            let task = pool.spawn(async move {
                let chunk = gen_chunk(chunk_loading_info.pos, &noise);
                chunk
            });
            (chunk_loading_info.pos, task)
        });
    chunk_loading_tasks.extend(new_tasks);
}

fn spawn_loaded_chunks(
    world: Res<World>,
    mut chunk_event_writer: EventWriter<ChunkUpdateEvent>,
    mut chunk_loading_tasks: ResMut<ChunkLoadingTasks>,
) {
    chunk_loading_tasks.retain(|pos, task| {
        let chunk: Option<Chunk> = block_on(poll_once(task));
        match chunk {
            None => {
                true
            }
            Some(chunk) => {
                world.add_chunk(*pos, chunk);
                chunk_event_writer.send(ChunkUpdateEvent::Loaded(*pos));
                false
            }
        }
    })
}
