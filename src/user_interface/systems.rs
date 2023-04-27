use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{
    game::resources::{AppState, GameAsset},
    set::{
        components::Background,
        systems::{spawn_obstacles, spawn_space_barriers},
    },
};

use super::components::*;

pub fn spawn_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_loading_screen(&mut commands, &asset_server);
}

pub fn build_loading_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            // screen background
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::PINK.into(),
                ..default()
            },
            Loading {},
        ))
        .with_children(|parent| {
            // box container for loading text
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(400.0), Val::Px(120.0)),
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // loading text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Loading...",
                                TextStyle {
                                    font: asset_server
                                        .load("fonts/PressStart2P.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<Loading>>,
) {
    if let Ok(loading_screen) = query.get_single() {
        commands.entity(loading_screen).despawn_recursive();
    }
}

pub fn spawn_menu_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_menu_screen(&mut commands, &asset_server);
}

pub fn build_menu_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            // screen background
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::YELLOW.into(),
                ..default()
            },
            Menu {},
        ))
        .with_children(|parent| {
            // box container for main menu
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(400.0), Val::Px(300.0)),
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // menu text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server
                                        .load("fonts/PressStart2P.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // start game button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    size: Size::new(
                                        Val::Px(200.0),
                                        Val::Px(80.0),
                                    ),

                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            },
                            PlayButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Play",
                                        TextStyle {
                                            font: asset_server
                                                .load("fonts/PressStart2P.ttf"),
                                            font_size: 32.0,
                                            color: Color::WHITE,
                                        },
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id()
}

pub fn despawn_menu_screen(
    mut commands: Commands,
    query: Query<Entity, With<Menu>>,
) {
    if let Ok(menu_screen) = query.get_single() {
        commands.entity(menu_screen).despawn_recursive();
    }
}

pub fn click_play_button(
    mut query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_state_next_state.set(AppState::Running);
            }
            _ => {
                // do nothing
            }
        }
    }
}

pub fn spawn_game_screen(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    build_game_screen(&mut commands, &window_query, &asset_server);

    spawn_space_barriers(&mut commands, &window_query);

    spawn_obstacles(&mut commands, &asset_server, &game_assets, &image_assets);
}

pub fn build_game_screen(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
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
            texture: asset_server.load("sprites/space/space.png"),
            ..default()
        },
        Background {},
    ));
}

pub fn despawn_game_screen(
    mut commands: Commands,
    query: Query<Entity, With<Running>>,
) {
    if let Ok(game_screen) = query.get_single() {
        commands.entity(game_screen).despawn_recursive();
    }
}

pub fn spawn_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_game_over_screen(&mut commands, &asset_server);
}

pub fn build_game_over_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            // screen background
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::ORANGE_RED.into(),
                ..default()
            },
            GameOver {},
        ))
        .with_children(|parent| {
            // box container for game over text
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(400.0), Val::Px(120.0)),
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // game over text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over!",
                                TextStyle {
                                    font: asset_server
                                        .load("fonts/PressStart2P.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOver>>,
) {
    if let Ok(game_screen) = query.get_single() {
        commands.entity(game_screen).despawn_recursive();
    }
}

pub fn detect_user_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        next_game_state.set(AppState::GameOver);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn play_menu_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio.play(asset_server.load("audio/menu_music.ogg"));
}
