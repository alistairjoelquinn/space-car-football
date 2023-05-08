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
pub struct GoalTimer {
    pub timer: Timer,
    pub color: Color,
}

impl Default for GoalTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
            color: Color::WHITE,
        }
    }
}
