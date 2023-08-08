use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::logic::world::world::World;
use crate::logic::world::{ChunkCoord, WorldAnchor};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<World>()
            .add_event::<ChunkUpdateEvent>()
            .add_systems(Update, load_chunks)
        ;
    }
}

#[derive(Component)]
struct ChunkEntity {
    pos: ChunkCoord,
}

#[derive(Event)]
pub enum ChunkUpdateEvent {
    Loaded(Entity, ChunkCoord),
    Unloaded,
}

/// Загружает чанки в память
fn load_chunks(
    mut commands: Commands,
    mut world: ResMut<World>,
    mut chunk_event_writer: EventWriter<ChunkUpdateEvent>,
    chunks_query: Query<(Entity, &ChunkEntity)>,
    world_anchors: Query<(&Transform, &WorldAnchor), With<WorldAnchor>>,
) {
    // Список чанков которые нужно удалить
    let mut chunks_to_unload = world.get_chunk_keys();

    // Список чанков которые нужно загрузить
    let mut chunks_to_load = HashSet::<ChunkCoord>::new();

    // Итерируемся по всем WorldAnchor
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
                    if !world.is_chunk_loaded(&pos) {
                        chunks_to_load.insert(pos);
                    }
                }
            }
        }
    }

    // Удаляем старые чанки
    for chunk_coord in chunks_to_unload.iter() {
        world.remove_chunk(chunk_coord);
        let (entity, _) = chunks_query.iter().find(|(_, pos)| pos.pos == *chunk_coord).unwrap();
        commands.entity(entity).despawn();
        chunk_event_writer.send(ChunkUpdateEvent::Unloaded)
    }

    // Загружаем новые чанки
    for chunk_coord in chunks_to_load {
        let entity = commands.spawn(ChunkEntity { pos: chunk_coord }).id();
        world.add_chunk(chunk_coord);
        chunk_event_writer.send(ChunkUpdateEvent::Loaded(entity, chunk_coord))
    }
}
