use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::game::resources::AppState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ball.in_schedule(OnExit(AppState::Menu)));
    }
}
