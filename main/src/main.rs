mod camera;
mod key_binding;
mod render;
mod logic;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::math::vec3;
use bevy::prelude::*;
use world_anchor::WorldAnchorPlugin;
use crate::camera::CameraPlugin;
use crate::key_binding::KeyBindingsPlugin;
use crate::render::{ChunkRenderPlugin, WorldMaterialPlugin};
use crate::logic::world::WorldPlugin;
use crate::render::debug::DebugInfoRenderPlugin;


fn main() {
    App::new()
        // Default bevy plugins setup
        .add_plugins(DefaultPlugins)

        // Additional bevy plugins setup
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EntityCountDiagnosticsPlugin)

        // Custom project plugins setup
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldAnchorPlugin)
        .add_plugins(WorldMaterialPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ChunkRenderPlugin)
        .add_plugins(DebugInfoRenderPlugin)

        .add_systems(Startup, setup)
        .run();
}

// TODO удалить
fn setup(
    mut commands: Commands,
) {

    // Спавним свет, пока у нас нет отдельного плагина под это дело, позже перенесу у другое место
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 128.0)
            .looking_at(vec3(90., 90., 60.), Vec3::Z),
        ..default()
    });
}
