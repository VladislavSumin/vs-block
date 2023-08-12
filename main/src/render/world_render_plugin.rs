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
            .init_resource::<WorldRenderState>()
            .add_systems(Startup, load_world_material)
            .add_systems(Update, update_chunk_mesh)
        ;
    }
}

#[derive(Resource)]
struct WorldMaterial {
    material_handle: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct WorldRenderState {
    /// Чанки которые необходимо отрендерить (или перерендерить)
    chunk_to_render: HashSet<ChunkPos>,

    /// Чанки которые необходимо выгрузить из памяти
    chunk_to_despawn: HashSet<ChunkPos>,

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

// TODO тестовая таска, переписать полностью
fn update_chunk_mesh(
    mut render_state: ResMut<WorldRenderState>,
    mut commands: Commands,
    mut chunk_event: EventReader<ChunkUpdateEvent>,
    world: Res<World>,
    mut assets: ResMut<Assets<Mesh>>,
    world_material: Res<WorldMaterial>,
) {
    // Обновляем состояние render_state добавляя туда чанки которые необходимо загрузить / выгрузить
    for event in chunk_event.iter() {
        match event {
            ChunkUpdateEvent::Loaded(pos) => {
                render_state.chunk_to_render.insert(*pos);
                render_state.chunk_to_despawn.remove(pos);
            }
            ChunkUpdateEvent::Unloaded(pos) => {
                render_state.chunk_to_render.remove(pos);
                render_state.chunk_to_despawn.insert(*pos);
            }
        }
    }

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

    // Пробуем удалить старые чанки
    let deleted_chunks: Vec<ChunkPos> = render_state.chunk_to_despawn.iter().filter_map(|pos| {
        let rendered_chunk = render_state.rendered_chunks.get(pos).unwrap_or(&None);
        match rendered_chunk {
            None => { Some(*pos) }
            Some(entity) => {
                commands.get_entity(*entity).map(|mut ec| {
                    ec.despawn();
                    *pos
                })
            }
        }
    }).collect();
    for pos in deleted_chunks {
        render_state.chunk_to_despawn.remove(&pos);
    }
}
