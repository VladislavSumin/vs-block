mod camera;
mod key_binding;
mod render;
mod logic;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::vec3;
use bevy::prelude::*;
use world_anchor::WorldAnchorPlugin;
use crate::camera::CameraPlugin;
use crate::key_binding::KeyBindingsPlugin;
use crate::render::ChunkRenderPlugin;
use crate::logic::world::WorldPlugin;
use crate::render::debug::DebugInfoRenderPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldAnchorPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ChunkRenderPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DebugInfoRenderPlugin)
        .add_systems(Startup, setup)
        .run();
}

// TODO удалить
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
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
