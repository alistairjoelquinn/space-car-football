use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use super::components::SpaceBarrier;

pub fn spawn_space_barriers(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // barriers which set on the edges of the game screen
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width() / 2., window.height() + 5., 9.),
        Vec2::new(window.width(), 10.),
        window.width(),
        5.,
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width() / 2., -5., 9.),
        Vec2::new(window.width(), 10.),
        window.width(),
        5.,
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(-5., window.height() / 2., 9.),
        Vec2::new(10., window.height()),
        5.,
        window.height(),
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width() + 5., window.height() / 2., 9.),
        Vec2::new(10., window.height()),
        5.,
        window.height(),
    );
}

fn create_space_barrier(
    commands: &mut Commands,
    translation: Vec3,
    size: Vec2,
    collider_x: f32,
    collider_y: f32,
) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(size),
                    ..Default::default()
                },
                ..default()
            },
            SpaceBarrier {},
        ))
        .insert(Collider::cuboid(collider_x, collider_y))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(1.0));
}

pub fn spawn_obstacles(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
}
