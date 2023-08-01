use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetClientPlugin)
        .run();
}
