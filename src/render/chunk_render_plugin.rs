use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder};
use crate::logic::chunk::Chunk;

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
    updated_chunks: Query<(Entity, &Chunk), Changed<Chunk>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, chunk) in updated_chunks.iter() {
        let mesh = assets.add(create_chunk_mesh(chunk));
        commands.entity(entity).insert(
            PbrBundle {
                mesh,
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(chunk.get_coordinates().get_absolute_coord()),
                ..default()
            }
        );
    }
}


// TODO вынести эту функцию отдельно
fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut builder = MeshBuilder::new();
    for (block_coord, block) in chunk.into_iter() {
        if let Some(_) = block {
            builder.set_transition(block_coord.into());
            builder.add_mesh_data(AbsoluteBlockFaceDirection::PosX);
            builder.add_mesh_data(AbsoluteBlockFaceDirection::NegX);
            builder.add_mesh_data(AbsoluteBlockFaceDirection::PosY);
            builder.add_mesh_data(AbsoluteBlockFaceDirection::NegY);
            builder.add_mesh_data(AbsoluteBlockFaceDirection::PosZ);
            builder.add_mesh_data(AbsoluteBlockFaceDirection::NegZ);
        }
    }
    builder.build()
}