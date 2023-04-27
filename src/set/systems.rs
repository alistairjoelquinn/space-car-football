use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::single_convex_polyline_collider_translated;

use crate::{game::resources::GameAsset, player::RIGHT};

use super::components::SpaceBarrier;

pub fn spawn_space_barriers(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // barriers which set on the edges of the game screen
    create_space_barrier(
        commands,
        Vec3::new(window.width() / 2., window.height() + 5., 9.),
        Vec2::new(window.width(), 10.),
        window.width(),
        5.,
    );
    create_space_barrier(
        commands,
        Vec3::new(window.width() / 2., -5., 9.),
        Vec2::new(window.width(), 10.),
        window.width(),
        5.,
    );
    create_space_barrier(
        commands,
        Vec3::new(-5., window.height() / 2., 9.),
        Vec2::new(10., window.height()),
        5.,
        window.height(),
    );
    create_space_barrier(
        commands,
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
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_assets: &Res<GameAsset>,
    image_assets: &Res<Assets<Image>>,
) {
    // spawn meteor
    let meteor_handle = game_assets.image_handles.get("meteor_handle");
    if meteor_handle.is_none() {
        return;
    }
    let meteor_image = image_assets.get(meteor_handle.unwrap()).unwrap();
    let meteor_collider =
        single_convex_polyline_collider_translated(meteor_image).unwrap();

    commands
        .spawn((
            meteor_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(400.0, 200.0, 10.0),
                    rotation: Quat::from_rotation_z(RIGHT),
                    scale: Vec3::splat(0.6),
                    ..default()
                },
                texture: asset_server.load("sprites/space/meteor_1.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);

    // spawn space station
    let space_station_large_handle =
        game_assets.image_handles.get("space_station_large_handle");
    if space_station_large_handle.is_none() {
        return;
    }
    let space_station_large_image = image_assets
        .get(space_station_large_handle.unwrap())
        .unwrap();
    let space_station_large_collider =
        single_convex_polyline_collider_translated(space_station_large_image)
            .unwrap();

    commands
        .spawn((
            space_station_large_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(700.0, 500.0, 10.0),
                    rotation: Quat::from_rotation_z(RIGHT),
                    scale: Vec3::splat(0.6),
                    ..default()
                },
                texture: asset_server
                    .load("sprites/space/space_station_large.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);

    // spawn meteor
    let rocket_3_handle = game_assets.image_handles.get("rocket_3_handle");
    if rocket_3_handle.is_none() {
        return;
    }
    let rocket_3_image = image_assets.get(rocket_3_handle.unwrap()).unwrap();
    let rocket_3_collider =
        single_convex_polyline_collider_translated(rocket_3_image).unwrap();

    commands
        .spawn((
            rocket_3_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(800.0, 100.0, 10.0),
                    rotation: Quat::from_rotation_z(PI + FRAC_PI_2),
                    scale: Vec3::splat(0.6),
                    ..default()
                },
                texture: asset_server.load("sprites/space/rocket_3.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);
}
