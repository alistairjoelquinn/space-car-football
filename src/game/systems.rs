use std::collections::HashMap;

use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};

use super::components::RpgSpriteHandles;
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

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    mut game_assets: ResMut<GameAsset>,
) {
    rpg_sprite_handles.handles = asset_server.load_folder("textures").unwrap();

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
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    mut state: ResMut<NextState<AppState>>,
) {
    let mut images_ready = false;
    let mut textures_ready = false;

    for image_handle in game_assets.image_handles.values() {
        if asset_server.get_load_state(image_handle) != LoadState::Loaded {
            images_ready = true;
        }
    }

    if let LoadState::Loaded = asset_server.get_group_load_state(
        rpg_sprite_handles.handles.iter().map(|handle| handle.id()),
    ) {
        textures_ready = true;
    }

    if images_ready == true && textures_ready == true {
        state.set(AppState::Menu);
    }
}
