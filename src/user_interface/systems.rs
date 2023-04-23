use bevy::prelude::*;

use super::components::Loading;

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
    let loading_screen = commands
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
        .id();
    loading_screen
}

pub fn remove_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<Loading>>,
) {
    if let Ok(loading_screen) = query.get_single() {
        commands.entity(loading_screen).despawn_recursive();
    }
}

// pub mod create_menu_screen() {};

// pub mod create_game_screen() {};

// pub mod create_game_over_screen() {};
