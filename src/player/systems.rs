use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

pub const PLAYER_SPEED: f32 = 300.0;

/// The rotation (in radians) for a sprite to faceØ north
pub const NORTH: f32 = FRAC_PI_2;
/// The rotation (in radians) for a sprite to face north east
pub const NORTH_EAST: f32 = FRAC_PI_4;
/// The rotation (in radians) for a sprite to face east
pub const EAST: f32 = 0.0;
/// The rotation (in radians) for a sprite to face south east
pub const SOUTH_EAST: f32 = PI + FRAC_PI_2 + FRAC_PI_4;
/// The rotation (in radians) for a sprite to face south
pub const SOUTH: f32 = PI + FRAC_PI_2;
/// The rotation (in radians) for a sprite to face south west
pub const SOUTH_WEST: f32 = PI + FRAC_PI_4;
/// The rotation (in radians) for a sprite to face west
pub const WEST: f32 = PI;
/// The rotation (in radians) for a sprite to face north west
pub const NORTH_WEST: f32 = FRAC_PI_2 + FRAC_PI_4;

pub fn spawn_players(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100.0, window.height() / 2.0, 0.0),
                rotation: Quat::from_rotation_z(EAST),
                ..default()
            },
            texture: asset_server.load("sprites/car_one.png"),
            ..default()
        },
        Player { id: 1, score: 0 },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    window.width() - 100.0,
                    window.height() / 2.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(EAST),
                ..default()
            },
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

pub fn set_window_boundary(
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 55.0;
        let x_max = window.width() - 55.0;
        let y_min = 20.00;
        let y_max = window.height() - 25.0;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
