use bevy::{prelude::*, window::PrimaryWindow};

use super::components::SpaceBarrier;

pub fn spawn_space_barriers(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    0.0,
                ),

                ..default()
            },
            texture: asset_server.load("sprites/space.png"),
            ..default()
        },
        SpaceBarrier {},
    ));
}
