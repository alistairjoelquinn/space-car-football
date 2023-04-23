use bevy::prelude::*;

use crate::game::resources::AppState;

use super::components::{Loading, Menu, PlayButton};

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
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
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
                            Menu {},
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

pub fn spawn_game_screen(mut commands: Commands) {
    build_game_screen(&mut commands);
}

pub fn build_game_screen(commands: &mut Commands) -> Entity {
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
        .id()
}

pub fn despawn_game_screen(
    mut commands: Commands,
    query: Query<Entity, With<Loading>>,
) {
    if let Ok(game_screen) = query.get_single() {
        commands.entity(game_screen).despawn_recursive();
    }
}
