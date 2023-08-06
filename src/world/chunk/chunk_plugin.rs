use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;
use crate::world::chunk::Chunk;

/// Отвечает за загрузку и выгрузку [Chunk], а так же за их генерацию
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generate_initial_chunk);
    }
}

fn generate_initial_chunk(mut commands: Commands) {
    commands.spawn(Chunk::new(32));
}