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

    // Список чанков которые нужно удалить
    let chunks_to_unload = &mut world.get_chunk_keys().clone();

    // Список чанков которые нужно загрузить
    let mut chunks_to_load = HashSet::<ChunkCoord>::new();

    for (transform, world_anchor) in world_anchors.iter() {
        let load_radius = world_anchor.load_radius as i32;

        // Получаем координаты чанка в котором находится WorldAnchor
        let mut anchor_chunk_coord = transform.translation;
        anchor_chunk_coord.z = 0.;
        let anchor_chunk_coord = ChunkCoord::from_global_coord(anchor_chunk_coord);

        for x in anchor_chunk_coord.raw_pos().x - load_radius..anchor_chunk_coord.raw_pos().x + load_radius {
            for y in anchor_chunk_coord.raw_pos().y - load_radius..anchor_chunk_coord.raw_pos().y + load_radius {
                let pos = ChunkCoord::new(x, y, 0);
                // Удаляем чанк находящийся внутри радиуса из списка чанков на удаление
                if !chunks_to_unload.remove(&pos) {
                    // Если такого чанка вообще не было среди загруженных, добавляем его в очередь на загрузку
                    chunks_to_load.insert(pos);
                }
            }
        }
    }

    // Удалем старые чанки
    for chunk_coord in chunks_to_unload.iter() {
        let entity = world.unload_chunk_if_exists(chunk_coord).unwrap();
        commands.entity(entity).despawn();
    }


    // Загружаем новые чанки
    for chunk_coord in chunks_to_load {
        let entity_factory = || {
            commands.spawn((
                ChunkEntity,
                chunk_coord
            ))
                .id()
        };
        world.load_chunk_if_not_loaded(chunk_coord, entity_factory);
    }
}
