use std::collections::HashMap;

use bevy::window::PrimaryWindow;
use bevy::{asset::LoadState, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::ball::components::Ball;
use crate::ball::systems::reset_ball_location;
use crate::game::resources::{AppState, GameAsset};
use crate::player::components::Player;
use crate::set::components::Goal;
use crate::user_interface::components::{Hud, ScoreText};

use super::resources::Score;

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
        (
            "meteor_1_handle".into(),
            asset_server.load("sprites/space/meteor_1.png"),
        ),
        (
            "earth_handle".into(),
            asset_server.load("sprites/space/planet_1.png"),
        ),
        (
            "robot_handle".into(),
            asset_server.load("sprites/space/robot.png"),
        ),
        (
            "rocket_3_handle".into(),
            asset_server.load("sprites/space/rocket_3.png"),
        ),
        (
            "space_station_large_handle".into(),
            asset_server.load("sprites/space/space_station_large.png"),
        ),
        (
            "space_station_small_handle".into(),
            asset_server.load("sprites/space/space_station_small.png"),
        ),
        (
            "grey_block_handle".into(),
            asset_server.load("sprites/space/grey_block.png"),
        ),
        (
            "red_dot_handle".into(),
            asset_server.load("sprites/space/red_dot.png"),
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

pub fn handle_collision_sounds(
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let mut just_collided = false;
    for pair in rapier_context.contact_pairs() {
        if pair.has_any_active_contacts() {
            just_collided = true;
        }
    }
    if just_collided {
        let sound = asset_server.load("audio/item_hit_2.ogg");
        audio.play(sound);
    }
}

pub fn handle_user_goal(
    rapier_context: Res<RapierContext>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ball_query: Query<(Entity, &mut Transform), With<Ball>>,
    goal_query: Query<(Entity, &Goal)>,
    player_query: Query<&Player>,
    mut score: ResMut<Score>,
) {
    let (ball_entity, mut ball_transform) = ball_query.single_mut();

    for (goal_entity, goal) in goal_query.iter() {
        if rapier_context.intersection_pair(goal_entity, ball_entity)
            == Some(true)
        {
            for player in player_query.iter() {
                if player.id == goal.user_id {
                    if player.id == 1 {
                        score.player_one += 1;
                    } else {
                        score.player_two += 1;
                    }
                    reset_ball_location(&window_query, &mut ball_transform);
                }
            }
        }
    }
}

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                    ..default()
                },
                ..default()
            },
            Hud {},
        ))
        .with_children(|parent| {
            // Score containers for both players
            create_hud_score_container(parent, &asset_server);
            create_hud_score_container(parent, &asset_server);
        });
}

fn create_hud_score_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(20.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            TextStyle {
                                font: asset_server
                                    .load("fonts/PressStart2P.ttf"),
                                font_size: 60.,
                                color: Color::WHITE,
                            },
                        )],
                        ..default()
                    },
                    ..default()
                },
                ScoreText,
            ));
        });
}

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for (i, mut text) in text_query.iter_mut().enumerate() {
            if i == 0 {
                text.sections[0].value =
                    format!("{}", score.player_one.to_string());
            } else {
                text.sections[0].value =
                    format!("{}", score.player_two.to_string());
            }
        }
    }
}

pub fn despawn_hud() {}
