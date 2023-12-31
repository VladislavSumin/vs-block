use std::sync::Arc;
use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::utils::{HashMap, HashSet};
use futures_lite::future::poll_once;
use futures_lite::future::block_on;
use strum::IntoEnumIterator;
use chunk::{ChunkNeighborDir, ChunkPos};
use crate::logic::world::{ChunkUpdateEvent, World};
use crate::render::chunk_mesh_builder::build_chunk_mesh;
use crate::render::world_material_plugin::WorldMaterial;

/// Отвечает за генерацию [Mesh] для загруженных [Chunk], а так же за обновление [Mesh] при
/// обновлении [Chunk]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldLoadChunksQueue>()
            .init_resource::<WorldUnloadChunksQueue>()
            .init_resource::<WorldRenderedChunks>()
            .init_resource::<WorldLoadChunksTasks>()
            .add_systems(Update, read_chunk_events)
            .add_systems(Update, start_load_chunks)
            .add_systems(Update, collect_loaded_chunks)
            .add_systems(Update, unload_chunks)
        ;
    }
}

/// Чанки которые необходимо выгрузить из памяти
///
/// Мы не можем обойтись без этой коллекции из-за асинхронной природы работы команд. Может произойти ситуация когда
/// мы сгенерировали меш для нового чанка и отдали bevy команду на спавн этого чанка в мир, но эта команда может не
/// успеть выполнится до получения команды на удаление этого чанка, поэтому необходимо сначала проверить была ли
/// entity в rendered_chunks на самом деле была сгенерена или еще нет
#[derive(Resource, Deref, DerefMut, Default)]
struct WorldUnloadChunksQueue(HashSet<ChunkPos>);

/// Чанки которые необходимо отрендерить ИЛИ чанки рендер которых нужно обновить
#[derive(Resource, Deref, DerefMut, Default)]
struct WorldLoadChunksQueue(HashSet<ChunkPos>);

#[derive(Resource, Deref, DerefMut, Default)]
struct WorldLoadChunksTasks(HashMap<ChunkPos, Task<Mesh>>);

/// Отрендеренные чанки.
///
/// Если по ключу присутствует значение, это означает, что чанк был отрендерен, при этом само значение optional так
/// как мы не создаем Entity если в результате оптимизации меша чанка он получился пустым (иначе это сильно
/// ухудшает общую производительность)
#[derive(Resource, Default, Deref, DerefMut)]
struct WorldRenderedChunks(HashMap<ChunkPos, Option<Entity>>);

/// Читает события [ChunkUpdateEvent] и управляет очередью загрузки/выгрузки чанков
fn read_chunk_events(
    rendered_chunks: ResMut<WorldRenderedChunks>,
    mut world_load_chunks_queue: ResMut<WorldLoadChunksQueue>,
    mut world_load_chunks_tasks: ResMut<WorldLoadChunksTasks>,
    mut world_unload_chunks_queue: ResMut<WorldUnloadChunksQueue>,
    mut chunk_event: EventReader<ChunkUpdateEvent>,
) {
    // Обновляем состояние render_state добавляя туда чанки которые необходимо загрузить / выгрузить
    for event in chunk_event.iter() {
        match event {
            ChunkUpdateEvent::Loaded(pos) => {
                world_load_chunks_queue.insert(*pos);
                if let Some(task) = world_load_chunks_tasks.remove(pos) {
                    let _ = task.cancel();
                }

                world_unload_chunks_queue.remove(pos);

                // Обновляем все чанки вокруг ново загрузившегося если они уже загружены
                for dir in ChunkNeighborDir::iter() {
                    if rendered_chunks.contains_key(&(*pos + dir))
                        || world_load_chunks_tasks.contains_key(&(*pos + dir))
                    {
                        world_load_chunks_queue.insert(*pos + dir);
                        if let Some(task) = world_load_chunks_tasks.remove(pos) {
                            let _ = task.cancel();
                        }
                    }
                }
            }
            ChunkUpdateEvent::Unloaded(pos) => {
                world_load_chunks_queue.remove(pos);
                if let Some(task) = world_load_chunks_tasks.remove(pos) {
                    let _ = task.cancel();
                }
                world_unload_chunks_queue.insert(*pos);
            }
        }
    }
}

fn start_load_chunks(
    mut world_load_chunks_queue: ResMut<WorldLoadChunksQueue>,
    mut world_load_chunks_tasks: ResMut<WorldLoadChunksTasks>,
    world: Res<World>,
) {
    let pool = AsyncComputeTaskPool::get();
    world_load_chunks_queue.retain(|pos| {
        let chunk = world.get_chunk(&pos);
        let chunk = match chunk {
            None => { return true; }
            Some(chunk) => { chunk }
        };
        let chunk_map = Arc::clone(&world.chunk_map);
        let pos = *pos;
        let task = pool.spawn(async move {
            let chunk = chunk.read().unwrap();
            build_chunk_mesh(&chunk_map, &chunk, pos)
        });
        world_load_chunks_tasks.insert(pos, task);
        false
    });
}


/// Создает меши новых чанков и загружает их в память
fn collect_loaded_chunks(
    mut world_load_chunks_tasks: ResMut<WorldLoadChunksTasks>,
    mut rendered_chunks: ResMut<WorldRenderedChunks>,
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
    world_material: Res<WorldMaterial>,
) {
    world_load_chunks_tasks.retain(|pos, task| {
        if !task.is_finished() { return true; }

        let entity = rendered_chunks.get(pos).unwrap_or(&None);

        if let Some(entity) = entity {
            let ec = commands.get_entity(*entity);
            match ec {
                None => { return true; }
                Some(mut ec) => {
                    rendered_chunks.remove(pos).unwrap();
                    ec.despawn()
                }
            }
        }

        let mesh: Mesh = block_on(poll_once(task)).unwrap();

        // Не спавним пустые меши, это сильно бьет по производительности рендера
        if mesh.indices().map(|indexes| { indexes.is_empty() }).unwrap_or(true) {
            rendered_chunks.insert(*pos, None);
            return false;
        }

        let mesh = assets.add(mesh);

        let bundle = PbrBundle {
            mesh,
            material: world_material.clone(),
            transform: Transform::from_translation(pos.get_absolute_coord().as_vec3()),
            ..default()
        };

        let entity = commands.spawn(bundle).id();

        rendered_chunks.insert(*pos, Some(entity));
        false
    });
}

/// Выгружает ненужные меши чанков из памяти
fn unload_chunks(
    mut world_unload_chunks_queue: ResMut<WorldUnloadChunksQueue>,
    mut rendered_chunks: ResMut<WorldRenderedChunks>,
    mut commands: Commands,
) {
    world_unload_chunks_queue.retain(|pos| {
        let rendered_chunk = rendered_chunks.get(pos);
        match rendered_chunk {
            None => {
                // Этот кейс означает что чанк был добавлен в очередь на загрузку, а после был удален еще до того как
                // загрузка чанка успела завершиться. В этом случае просто удаляем чанк из очереди
                false
            }
            Some(entity_option) => {
                // Чанк был загружен (была отдана команда bevy на загрузку в память)
                match entity_option {
                    None => {
                        // Чанк не имеет меша, можно смело удалять
                        rendered_chunks.remove(pos).unwrap();
                        false
                    }
                    Some(entity) => {
                        let entity_commands = commands.get_entity(*entity);
                        match entity_commands {
                            None => {
                                // Ентити еще не успела загрузиться, попробуем удалить чанк позже
                                true
                            }
                            Some(mut entity_commands) => {
                                // Ентити загрузилась, удаляем
                                entity_commands.despawn();
                                rendered_chunks.remove(pos).unwrap();
                                false
                            }
                        }
                    }
                }
            }
        }
    });
}
