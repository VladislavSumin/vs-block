use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use strum::IntoEnumIterator;
use crate::logic::chunk::{Chunk, ChunkBlockPos};
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};
use crate::logic::world::{ChunkUpdateEvent, World};

/// Отвечает за генерацию [Mesh] для загруженных [Chunk], а так же за обновление [Mesh] при
/// обновлении [Chunk]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app
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
) {
    for event in chunk_event.iter() {
        if let ChunkUpdateEvent::Loaded(entity, chunk_coord) = event {
            let chunk = world.get_chunk(chunk_coord).unwrap();
            let mesh = assets.add(create_chunk_mesh(chunk));

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
                    transform: Transform::from_translation(chunk_coord.get_absolute_coord()),
                    ..default()
                }
            );
        }
    }
}


// TODO вынести эту функцию отдельно
fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut builder = MeshBuilder::new();
    for (block_coord, block) in chunk.into_iter() {
        if let Some(_) = block {
            builder.set_transition(block_coord.into());

            for dir in AbsoluteBlockFaceDirection::iter() {
                if !has_neighbor_block(chunk, block_coord, dir) {
                    builder.add_mesh_data(dir);
                }
            }
        }
    }
    builder.build()
}

fn has_neighbor_block(chunk: &Chunk, pos: ChunkBlockPos, dir: AbsoluteBlockFaceDirection) -> bool {
    let dir_pos: IVec3 = dir.into();
    let neighbor_pos: Result<ChunkBlockPos, ()> = (pos.as_ivec3() + dir_pos).try_into();
    if let Ok(pos) = neighbor_pos {
        chunk[&pos].is_some()
    } else {
        false
    }
}