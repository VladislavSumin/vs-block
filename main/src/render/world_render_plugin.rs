use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;
use chunk::AbsoluteBlockPos;
use crate::logic::chunk::{Chunk, ChunkBlockPos, ChunkMap, ChunkPos};
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};
use crate::logic::world::{ChunkUpdateEvent, World};

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
                let mesh = assets.add(create_chunk_mesh(&world.chunk_map, chunk, *pos));

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


// TODO вынести эту функцию отдельно
fn create_chunk_mesh(map: &ChunkMap, chunk: &Chunk, chunk_pos: ChunkPos) -> Mesh {
    let mut builder = MeshBuilder::new();
    for (block_coord, block) in chunk.into_iter() {
        if let Some(_) = block {
            builder.set_transition(block_coord.into());

            for dir in AbsoluteBlockFaceDirection::iter() {
                if !has_neighbor_block(map, chunk, chunk_pos, block_coord, dir) {
                    builder.add_mesh_data(dir);
                }
            }
        }
    }
    builder.build()
}

fn has_neighbor_block(map: &ChunkMap, chunk: &Chunk, chunk_pos: ChunkPos, pos: ChunkBlockPos, dir: AbsoluteBlockFaceDirection) -> bool {
    // Вычисляем глобальные координаты блока соседствующего с данным с переданной стороны
    let dir_pos: IVec3 = dir.into();
    let global_pos: AbsoluteBlockPos = (chunk_pos.get_absolute_coord() + pos.as_ivec3() + dir_pos).into();

    if let Ok(local_pos) = chunk_pos.try_global_pos_into_chunk_pos(global_pos) {
        chunk[&local_pos].is_some()
    } else {
        map.get_block_at(global_pos).is_some()
    }
}