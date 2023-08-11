use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::utils::HashMap;
use chunk::ChunkPos;
use crate::logic::world::{ChunkUpdateEvent, World};
use crate::render::chunk_mesh_builder::build_chunk_mesh;

/// Отвечает за генерацию [Mesh] для загруженных [Chunk], а так же за обновление [Mesh] при
/// обновлении [Chunk]
pub struct ChunkRenderPlugin;

#[derive(Default, Resource)]
struct RenderedChunksMap {
    chunks: HashMap<ChunkPos, Entity>,
}

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RenderedChunksMap>()
            .add_systems(Startup, load_world_material)
            .add_systems(Update, update_chunk_mesh)
        ;
    }
}

#[derive(Resource)]
struct WorldMaterial {
    material_handle: Handle<StandardMaterial>,
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
    mut commands: Commands,
    mut chunk_event: EventReader<ChunkUpdateEvent>,
    world: Res<World>,
    mut assets: ResMut<Assets<Mesh>>,
    mut rendered_chunks_map: ResMut<RenderedChunksMap>,
    world_material: Res<WorldMaterial>,
) {
    for event in chunk_event.iter() {
        match event {
            ChunkUpdateEvent::Loaded(entity, pos) => {
                rendered_chunks_map.chunks.insert(*pos, *entity);

                let chunk = world.get_chunk(pos).unwrap();
                let mesh = build_chunk_mesh(chunk, *pos);

                // Не спавним пустые меши, это сильно бьет по производительности рендера
                if mesh.indices().map(|indexes| { indexes.is_empty() }).unwrap_or(true) {
                    continue;
                }

                let mesh = assets.add(mesh);

                commands.entity(*entity).insert(
                    PbrBundle {
                        mesh,
                        material: world_material.material_handle.clone(),
                        transform: Transform::from_translation(pos.get_absolute_coord().as_vec3()),
                        ..default()
                    }
                );
            }
            ChunkUpdateEvent::Unloaded(pos) => {
                rendered_chunks_map.chunks.remove(pos);
            }
        }
    }
}
