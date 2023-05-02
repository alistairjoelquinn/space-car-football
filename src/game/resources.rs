use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Resource, Default)]
pub struct GameAsset {
    pub font_handle: Handle<Font>,
    pub image_handles: HashMap<String, Handle<Image>>,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub player_1: u32,
    pub player_2: u32,
}
