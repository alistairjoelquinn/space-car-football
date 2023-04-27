use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

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

    // spawn rocket
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

    // spawn grey block
    let grey_block_handle = game_assets.image_handles.get("grey_block_handle");
    if grey_block_handle.is_none() {
        return;
    }
    let grey_block_image =
        image_assets.get(grey_block_handle.unwrap()).unwrap();
    let grey_block_collider =
        single_convex_polyline_collider_translated(grey_block_image).unwrap();

    commands
        .spawn((
            grey_block_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(900.0, 250.0, 10.0),
                    rotation: Quat::from_rotation_z(PI + FRAC_PI_4),
                    scale: Vec3::splat(1.6),
                    ..default()
                },
                texture: asset_server.load("sprites/space/grey_block.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);

    // spawn red dot
    let red_dot_handle = game_assets.image_handles.get("red_dot_handle");
    if red_dot_handle.is_none() {
        return;
    }
    let red_dot_image = image_assets.get(red_dot_handle.unwrap()).unwrap();
    let red_dot_collider =
        single_convex_polyline_collider_translated(red_dot_image).unwrap();

    commands
        .spawn((
            red_dot_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(250.0, 550.0, 10.0),
                    scale: Vec3::splat(2.),
                    ..default()
                },
                texture: asset_server.load("sprites/space/red_dot.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);
}
