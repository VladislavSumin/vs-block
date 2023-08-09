use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Added, Changed, Commands, Entity, Query, Transform};
use chunk::ChunkPos;
use generic_assert::{Assert, IsTrue};
use crate::{WorldAnchor, WorldAnchorInChunkPos};

/// Плагин следит за сущностями с компонентом [WorldAnchor] и добавляет для них компонент [WorldAnchorInChunkPos]
pub struct WorldAnchorPlugin<const CHUNK_SIZE: usize> where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue;

impl<const CHUNK_SIZE: usize> Plugin for WorldAnchorPlugin<CHUNK_SIZE>
    where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_world_anchor_position::<CHUNK_SIZE>)
            .add_systems(Update, update_world_anchor_position::<CHUNK_SIZE>)
        ;
    }
}

/// Для каждого [WorldAnchor] добавляем внутреннюю сущность [WorldAnchorPos], это нужно для повышения производительности
/// других функций которым нужна позиция [WorldAnchor] применимо к сетке координат чанков
fn spawn_world_anchor_position<const CHUNK_SIZE: usize>(
    mut commands: Commands,
    new_world_anchors: Query<(Entity, &Transform), Added<WorldAnchor>>,
) where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    for (entity, transform) in new_world_anchors.iter() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            let pos = ChunkPos::from_global_coord(transform.translation);
            entity_commands.insert(WorldAnchorInChunkPos::<CHUNK_SIZE> { pos });
        }
    }
}

/// Обновляем [ChunkCoord] при изменении [Transform]
fn update_world_anchor_position<const CHUNK_SIZE: usize>(
    mut changed_world_anchors: Query<(&mut WorldAnchorInChunkPos<CHUNK_SIZE>, &Transform), Changed<Transform>>,
) where Assert<{ CHUNK_SIZE <= u8::MAX as usize }>: IsTrue {
    for (mut pos, transform) in changed_world_anchors.iter_mut() {
        let new_pos = ChunkPos::from_global_coord(transform.translation);

        // Bevy проверяет не изменился ли компонент по факту записи в переменную, а не посредствам equals
        if pos.pos != new_pos {
            pos.pos = new_pos
        }
    }
}