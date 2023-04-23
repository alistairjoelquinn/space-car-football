use bevy::{prelude::*, window::PrimaryWindow};

use super::components::SpaceBarrier;

pub fn spawn_space_barriers(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // barriers which set on the edges of the game screen
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width() / 2., window.height(), 9.),
        Vec2::new(window.width(), 10.),
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width() / 2., 0., 9.),
        Vec2::new(window.width(), 10.),
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(0., window.height() / 2., 9.),
        Vec2::new(10., window.height()),
    );
    create_space_barrier(
        &mut commands,
        Vec3::new(window.width(), window.height() / 2., 9.),
        Vec2::new(10., window.height()),
    );
}

fn create_space_barrier(
    commands: &mut Commands,
    translation: Vec3,
    size: Vec2,
) {
    commands.spawn((
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
    ));
}
