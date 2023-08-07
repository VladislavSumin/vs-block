use bevy::app::{App, Startup};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::fmt::Write;
use crate::camera::PlayerCamera;

/// Отображает дополнительную дебажную информацию
pub struct DebugInfoRenderPlugin;

impl Plugin for DebugInfoRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_debug_info)
            .add_systems(Update, update_debug_info)
        ;
    }
}

#[derive(Component)]
struct DebugText;

fn setup_debug_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font_mono.ttf");

    let text_bundle = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "FPS ...".to_string(),
                style: TextStyle {
                    font,
                    font_size: 24.,
                    color: Color::ORANGE,
                },
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn((
        text_bundle,
        DebugText,
    ));
}

fn update_debug_info(
    diagnostics: Res<DiagnosticsStore>,
    player_query: Query<&Transform, With<PlayerCamera>>,
    mut text_query: Query<&mut Text, With<DebugText>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
        .unwrap_or(0.) as u32;

    let player_coord = player_query.single().translation;

    let text = &mut text_query.single_mut().sections[0].value;
    text.clear();
    write!(
        text,
        "  FPS:{:3} x:{:.01} y:{:.01}, z:{:.01}",
        fps, player_coord.x, player_coord.y, player_coord.z
    )
        .unwrap();
}