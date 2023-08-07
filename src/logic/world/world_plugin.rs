use bevy::prelude::*;
use bevy::utils::HashSet;
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

/// Загружает чанки в память
fn load_chunks(
    mut world: ResMut<World>,
    world_anchors: Query<(&Transform, &WorldAnchor), With<WorldAnchor>>,
) {
    let mut chunks_to_load = HashSet::<ChunkCoord>::default();

    // TODO пока у нас один WorldAnchor, поэтому пока пишем алгоритм для работы с одним,
    // в будущем можно будет добавить поддержку работы с несколькими якорями

    let (transform, world_anchor) = world_anchors.single();
    let anchor_chunk_coord = ChunkCoord::from_global_coord(transform.translation);
    let load_radius = world_anchor.load_radius;


}
