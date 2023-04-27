use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::components::Ball;

pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 2.0,
                        window.height() / 2.0,
                        10.0,
                    ),
                    scale: Vec3::splat(0.1),
                    ..default()
                },
                texture: asset_server.load("sprites/football.png"),
                ..default()
            },
            Ball,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(250.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Restitution::coefficient(2.0))
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 0.2,
        })
        .insert(ActiveEvents::COLLISION_EVENTS);
}
