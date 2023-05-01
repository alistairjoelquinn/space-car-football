use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::single_convex_polyline_collider_translated;

use crate::game::resources::GameAsset;
use crate::player::RIGHT;
use crate::set::components::{Goal, Meteor, SpaceBarrier};

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
                    translation: Vec3::new(700., 500., 10.),
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
                    translation: Vec3::new(800., 100., 10.),
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
                    translation: Vec3::new(900., 250., 10.),
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
                    translation: Vec3::new(250., 550., 10.),
                    scale: Vec3::splat(2.),
                    ..default()
                },
                texture: asset_server.load("sprites/space/red_dot.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);

    // spawn robot
    let robot_handle = game_assets.image_handles.get("robot_handle");
    if robot_handle.is_none() {
        return;
    }
    let robot_image = image_assets.get(robot_handle.unwrap()).unwrap();
    let robot_collider =
        single_convex_polyline_collider_translated(robot_image).unwrap();

    commands
        .spawn((
            robot_collider,
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(400., 250., 10.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
                texture: asset_server.load("sprites/space/robot.png"),
                ..default()
            },
        ))
        .insert(RigidBody::Fixed);

    // spawn meteors
    for index in 1..=3 {
        let meteor_1_handle = game_assets.image_handles.get("meteor_1_handle");
        if meteor_1_handle.is_none() {
            return;
        }
        let meteor_1_image =
            image_assets.get(meteor_1_handle.unwrap()).unwrap();
        let meteor_1_collider =
            single_convex_polyline_collider_translated(meteor_1_image).unwrap();

        let meteor = Meteor {
            x: 0. + (index as f32 * 300.),
            y: 700. - (index as f32 * 200.),
            z: 10.,
        };

        commands
            .spawn((
                meteor_1_collider,
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(meteor.x, meteor.y, meteor.z),
                        scale: Vec3::splat(0.4),
                        ..default()
                    },
                    texture: asset_server.load("sprites/space/meteor_1.png"),
                    ..default()
                },
                meteor,
            ))
            .insert(RigidBody::Dynamic);
    }
}

pub fn set_meteor_window_boundary(
    mut query: Query<&mut Transform, With<Meteor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 35.;
        let x_max = window.width() - 35.;
        let y_min = 35.;
        let y_max = window.height() - 35.;

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

pub fn spawn_goals(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // goal for user 1
    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 2.,
                        window.height(),
                        9.,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(15., 150., 56.),
                    custom_size: Some(Vec2::new(25., window.height() / 2.)),
                    ..Default::default()
                },
                ..default()
            },
            Goal { user_id: 1 },
        ))
        .insert(Collider::cuboid(10., 50.))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(1.0));

    // goal for user 2
    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 3.,
                        window.height(),
                        9.,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(window.width(), 10.)),
                    ..Default::default()
                },
                ..default()
            },
            Goal { user_id: 2 },
        ))
        .insert(Collider::cuboid(10., 50.))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(1.));
}
