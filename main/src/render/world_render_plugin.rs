use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use chunk::ChunkPos;
use crate::logic::world::{ChunkUpdateEvent, World};
use crate::render::chunk_mesh_builder::build_chunk_mesh;

/// Отвечает за генерацию [Mesh] для загруженных [Chunk], а так же за обновление [Mesh] при
/// обновлении [Chunk]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldUnloadChunksQueue>()
            .init_resource::<WorldRenderState>()
            .add_systems(Startup, load_world_material)
            .add_systems(Update, read_chunk_events)
            .add_systems(Update, load_chunks)
            .add_systems(Update, unload_chunks)
        ;
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct WorldUnloadChunksQueue {
    /// Чанки которые необходимо выгрузить из памяти
    ///
    /// Мы не можем обойтись без этой коллекции из-за асинхронной природы работы команд. Может произойти ситуация когда
    /// мы сгенерировали меш для нового чанка и отдали bevy команду на спавн этого чанка в мир, но эта команда может не
    /// успеть выполнится до получения команды на удаление этого чанка, поэтому необходимо сначала проверить была ли
    /// entity в rendered_chunks на самом деле была сгенерена или еще нет
    chunk_to_despawn: HashSet<ChunkPos>,
}

#[derive(Resource)]
struct WorldMaterial {
    material_handle: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct WorldRenderState {
    /// Чанки которые необходимо отрендерить (или перерендерить)
    chunk_to_render: HashSet<ChunkPos>,

    /// Отрендеренные чанки.
    ///
    /// Если по ключу присутствует значение, это означает, что чанк был отрендере, при этом само значение optional так
    /// как мы не создаем Entity если в результате оптимизации меша чанка он получился пустым (иначе это сильно
    /// ухудшает общую производительность)
    rendered_chunks: HashMap<ChunkPos, Option<Entity>>,
}

fn load_world_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let texture: Handle<Image> = asset_server.load("dirt.png");

    let material = StandardMaterial {
        base_color_texture: Some(texture),
        unlit: false,
        metallic: 0.,
        reflectance: 0.,
        ..default()
    };

    let material_handle = materials.add(material);
    //
    let world_material = WorldMaterial {
        material_handle
    };

    commands.insert_resource(world_material);
}

/// Читает события [ChunkUpdateEvent] и управляет очередью загрузки/выгрузки чанков
fn read_chunk_events(
    mut render_state: ResMut<WorldRenderState>,
    mut world_unload_chunks_queue: ResMut<WorldUnloadChunksQueue>,
    mut chunk_event: EventReader<ChunkUpdateEvent>,
) {
    // Обновляем состояние render_state добавляя туда чанки которые необходимо загрузить / выгрузить
    for event in chunk_event.iter() {
        match event {
            ChunkUpdateEvent::Loaded(pos) => {
                render_state.chunk_to_render.insert(*pos);
                world_unload_chunks_queue.remove(pos);
            }
            ChunkUpdateEvent::Unloaded(pos) => {
                render_state.chunk_to_render.remove(pos);
                world_unload_chunks_queue.insert(*pos);
            }
        }
    }
}

/// Создает меши новых чанков и загружает их в память
fn load_chunks(
    mut render_state: ResMut<WorldRenderState>,
    mut commands: Commands,
    world: Res<World>,
    mut assets: ResMut<Assets<Mesh>>,
    world_material: Res<WorldMaterial>,
) {

    // Рендерим новые чанки и добавляем их rendered_chunks
    let new_chunks: Vec<(ChunkPos, Option<Entity>)> = render_state.chunk_to_render.drain().map(|pos| {
        let chunk = world.get_chunk(&pos).unwrap();
        let mesh = build_chunk_mesh(chunk, pos);

        // Не спавним пустые меши, это сильно бьет по производительности рендера
        if mesh.indices().map(|indexes| { indexes.is_empty() }).unwrap_or(true) {
            return (pos, None);
        }

        let mesh = assets.add(mesh);

        let entity = commands.spawn(
            PbrBundle {
                mesh,
                material: world_material.material_handle.clone(),
                transform: Transform::from_translation(pos.get_absolute_coord().as_vec3()),
                ..default()
            }
        ).id();

        (pos, Some(entity))
    }).collect();
    render_state.rendered_chunks.extend(new_chunks);
}

/// Выгружает ненужные меши чанков из памяти
fn unload_chunks(
    mut world_unload_chunks_queue: ResMut<WorldUnloadChunksQueue>,
    mut render_state: ResMut<WorldRenderState>,
    mut commands: Commands,
) {
    world_unload_chunks_queue.retain(|pos| {
        let rendered_chunk = render_state.rendered_chunks.get(pos);
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
                        render_state.rendered_chunks.remove(pos).unwrap();
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
                                render_state.rendered_chunks.remove(pos).unwrap();
                                false
                            }
                        }
                    }
                }
            }
        }
    });
}