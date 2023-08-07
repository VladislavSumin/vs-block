use bevy::prelude::*;
use crate::logic::world::world::World;
use crate::logic::world::{ChunkCoord, WorldAnchor};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<World>()
            .add_systems(Update, load_chunks)
        ;
    }
}

/// Маркер, что [Entity] является чанком
#[derive(Component)]
pub struct ChunkEntity;

/// Загружает чанки в память
fn load_chunks(
    mut commands: Commands,
    mut world: ResMut<World>,
    world_anchors: Query<(&Transform, &WorldAnchor), With<WorldAnchor>>,
) {
    // TODO пока у нас один WorldAnchor, поэтому пока пишем алгоритм для работы с одним,
    // в будущем можно будет добавить поддержку работы с несколькими якорями

    let (transform, world_anchor) = world_anchors.single();
    let mut anchor_chunk_coord = ChunkCoord::from_global_coord(transform.translation);
    anchor_chunk_coord.z = 0;

    // TODO поддержать load_radius
    let load_radius = world_anchor.load_radius;

    let entity_factory = || {
        commands.spawn((
            ChunkEntity,
            anchor_chunk_coord
        ))
            .id()
    };


    world.load_chunk_if_not_loaded(anchor_chunk_coord, entity_factory);
}
