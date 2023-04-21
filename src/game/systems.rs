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
            0.0,
        ),

        ..default()
    });
}

pub fn spawn_set(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_set(&mut commands, &asset_server);
}

pub fn build_set(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands.spawn(
        (NodeBundle {
            style: {},
            ..default()
        }),
    )
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
    println!("Checking assets");
    for h in game_assets.image_handles.values() {
        if LoadState::Loaded != asset_server.get_load_state(h) {
            return;
        }
    }

    println!("Assets loaded");
    state.set(AppState::Running)
}
