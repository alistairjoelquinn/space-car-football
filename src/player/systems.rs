use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

// pub const PLAYER_SPEED: f32 = 300.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/car_one.png"),
            ..default()
        },
        Player { score: 0 },
    ));
}
