use std::collections::HashMap;

use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};

use super::resources::{AppState, GameAsset};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            10.0,
        ),
        ..default()
    });
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAsset>,
) {
    game_assets.image_handles = HashMap::from([
        (
            "car_one_handle".into(),
            asset_server.load("sprites/car_one.png"),
        ),
        (
            "car_two_handle".into(),
            asset_server.load("sprites/car_two.png"),
        ),
        (
            "ball_handle".into(),
            asset_server.load("sprites/football.png"),
        ),
    ]);
}

pub fn check_assets(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAsset>,
    mut state: ResMut<NextState<AppState>>,
) {
    for image_handle in game_assets.image_handles.values() {
        if asset_server.get_load_state(image_handle) != LoadState::Loaded {
            return;
        }
    }

    state.set(AppState::Menu);
}
