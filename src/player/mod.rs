pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::game::resources::AppState;
use crate::player::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_players.in_schedule(OnExit(AppState::Menu)),
            player_movement.run_if(in_state(AppState::Running)),
            set_window_boundary.run_if(in_state(AppState::Running)),
        ));
    }
}
