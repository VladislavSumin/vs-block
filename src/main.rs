mod camera;
mod key_binding;
mod render;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::key_binding::KeyBindingsPlugin;
use crate::render::{AbsoluteBlockFaceDirection, MeshBuilder, MeshPart};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Дефолтный пример отсюда https://bevyengine.org/examples/3D%20Rendering/3d-scene/
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // сторона куба, это почти целый куб, только нужно еще 5 сторон
    commands.spawn(PbrBundle {
        mesh: meshes.add(create_block_mesh()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn create_block_mesh() -> Mesh {
    let mut builder = MeshBuilder::new();
    builder.add_mesh_data(AbsoluteBlockFaceDirection::PosX);
    builder.add_mesh_data(AbsoluteBlockFaceDirection::NegX);
    builder.add_mesh_data(AbsoluteBlockFaceDirection::PosY);
    builder.add_mesh_data(AbsoluteBlockFaceDirection::NegY);
    builder.add_mesh_data(AbsoluteBlockFaceDirection::PosZ);
    builder.add_mesh_data(AbsoluteBlockFaceDirection::NegZ);
    builder.build()
}
