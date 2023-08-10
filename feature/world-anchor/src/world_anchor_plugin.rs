use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Added, Changed, Commands, Entity, Query, Transform};
use chunk::ChunkPos;
use crate::{WorldAnchor, WorldAnchorInChunkPos};

/// Плагин следит за сущностями с компонентом [WorldAnchor] и добавляет для них компонент [WorldAnchorInChunkPos]
pub struct WorldAnchorPlugin;

impl Plugin for WorldAnchorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_world_anchor_position)
            .add_systems(Update, update_world_anchor_position)
        ;
    }
}

/// Для каждого [WorldAnchor] добавляем внутреннюю сущность [WorldAnchorPos], это нужно для повышения производительности
/// других функций которым нужна позиция [WorldAnchor] применимо к сетке координат чанков
fn spawn_world_anchor_position(
    mut commands: Commands,
    new_world_anchors: Query<(Entity, &Transform), Added<WorldAnchor>>,
) {
    for (entity, transform) in new_world_anchors.iter() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            let pos = ChunkPos::from_global_coord(transform.translation.as_ivec3());
            entity_commands.insert(WorldAnchorInChunkPos { pos });
        }
    }
}

/// Обновляем [ChunkCoord] при изменении [Transform]
fn update_world_anchor_position(
    mut changed_world_anchors: Query<(&mut WorldAnchorInChunkPos, &Transform), Changed<Transform>>,
) {
    for (mut pos, transform) in changed_world_anchors.iter_mut() {
        let new_pos = ChunkPos::from_global_coord(transform.translation.as_ivec3());

        // Bevy проверяет не изменился ли компонент по факту записи в переменную, а не посредствам equals
        if pos.pos != new_pos {
            pos.pos = new_pos
        }
    }
}