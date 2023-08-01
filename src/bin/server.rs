use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .run();
}
