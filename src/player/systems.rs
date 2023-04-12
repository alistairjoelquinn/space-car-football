use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

// pub const PLAYER_SPEED: f32 = 300.0;

pub fn spawn_players(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // spawn player 1
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/car_one.png"),
            ..default()
        },
        Player { id: 1, score: 0 },
    ));

    // spawn player 2
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() - 100.0,
                window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/car_two.png"),
            ..default()
        },
        Player { id: 2, score: 0 },
    ));
}
