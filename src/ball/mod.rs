pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::game::resources::AppState;
use systems::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ball.in_schedule(OnExit(AppState::Menu)));
    }
}
