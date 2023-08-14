use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct WorldMaterialPlugin;

impl Plugin for WorldMaterialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_world_material)
        ;
    }
}

#[derive(Resource, Deref)]
pub struct WorldMaterial {
    material_handle: Handle<StandardMaterial>,
}

fn load_world_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let texture: Handle<Image> = asset_server.load("dirt.png");

    let material = StandardMaterial {
        base_color_texture: Some(texture),
        unlit: false,
        metallic: 0.,
        reflectance: 0.,
        ..default()
    };

    let material_handle = materials.add(material);

    let world_material = WorldMaterial {
        material_handle
    };

    commands.insert_resource(world_material);
}