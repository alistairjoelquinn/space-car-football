use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::components::Player;
use super::*;

pub const PLAYER_SPEED: f32 = 300.0;

pub fn spawn_players(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(100.0, window.height() / 2.0, 0.0),
                    rotation: Quat::from_rotation_z(RIGHT),
                    ..default()
                },
                texture: asset_server.load("sprites/car_one.png"),
                ..default()
            },
            Player { id: 1, score: 0 },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0));

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        window.width() - 100.0,
                        window.height() / 2.0,
                        0.0,
                    ),
                    rotation: Quat::from_rotation_z(RIGHT),
                    ..default()
                },
                texture: asset_server.load("sprites/car_two.png"),
                ..default()
            },
            Player { id: 2, score: 0 },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut rotation = transform.rotation;

        if player.id == 1 {
            if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
                rotation = Quat::from_rotation_z(RIGHT);
                if keyboard_input.pressed(KeyCode::W) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::S) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
                rotation = Quat::from_rotation_z(RIGHT);
                if keyboard_input.pressed(KeyCode::W) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::S) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);
                if keyboard_input.pressed(KeyCode::A) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::D) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);
                if keyboard_input.pressed(KeyCode::D) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::A) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }
        } else if player.id == 2 {
            if keyboard_input.pressed(KeyCode::Left) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
                rotation = Quat::from_rotation_z(RIGHT);
                if keyboard_input.pressed(KeyCode::Up) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::Right) {
                direction += Vec3::new(1.0, 0.0, 0.0);
                rotation = Quat::from_rotation_z(RIGHT);
                if keyboard_input.pressed(KeyCode::Up) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::Up) {
                direction += Vec3::new(0.0, 1.0, 0.0);
                if keyboard_input.pressed(KeyCode::Left) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }

            if keyboard_input.pressed(KeyCode::Down) {
                direction += Vec3::new(0.0, -1.0, 0.0);
                if keyboard_input.pressed(KeyCode::Right) {
                    rotation = Quat::from_rotation_z(BOTTOM_RIGHT);
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    rotation = Quat::from_rotation_z(TOP_RIGHT);
                }
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation +=
            direction * PLAYER_SPEED * time.delta_seconds();
        transform.rotation = rotation;
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
