use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;
use chunk::{AbsoluteBlockPos, ChunkBlockPos, ChunkPos};
use crate::logic::chunk::{Chunk, ChunkMap};
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};
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
            .add_systems(Update, update_chunk_mesh);
    }
}

// TODO тестовая таска, переписать полностью
fn update_chunk_mesh(
    mut commands: Commands,
    mut chunk_event: EventReader<ChunkUpdateEvent>,
    world: Res<World>,
    mut assets: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rendered_chunks_map: ResMut<RenderedChunksMap>,
) {
    for event in chunk_event.iter() {
        match event {
            ChunkUpdateEvent::Loaded(entity, pos) => {
                rendered_chunks_map.chunks.insert(*pos, *entity);

                let chunk = world.get_chunk(pos).unwrap();
                let mesh = assets.add(build_chunk_mesh(chunk, *pos));

                let texture: Handle<Image> = asset_server.load("dirt.png");

                let material = StandardMaterial {
                    base_color_texture: Some(texture),
                    unlit: false,
                    metallic: 0.,
                    reflectance: 0.,
                    ..default()
                };

                commands.entity(*entity).insert(
                    PbrBundle {
                        mesh,
                        material: materials.add(material),
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