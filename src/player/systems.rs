use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

pub const PLAYER_SPEED: f32 = 300.0;

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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if player.id == 1 {
            if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
        } else if player.id == 2 {
            if keyboard_input.pressed(KeyCode::Left) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::Right) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::Up) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::Down) {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation +=
            direction * PLAYER_SPEED * time.delta_seconds();
    }
}
