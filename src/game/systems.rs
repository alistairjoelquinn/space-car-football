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
    _asset_server: &Res<AssetServer>,
) -> Entity {
    let set = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                ..default()
            },
            background_color: Color::AZURE.into(),
            z_index: ZIndex::Global(0),
            ..default()
        })
        .id();
    set
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAsset>,
) {
    println!("Game is loading.........");
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
    for image_handle in game_assets.image_handles.values() {
        if asset_server.get_load_state(image_handle) != LoadState::Loaded {
            return;
        }
    }

    println!("Assets loaded");
    state.set(AppState::Running)
}
