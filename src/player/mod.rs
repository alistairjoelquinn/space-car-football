use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

pub mod colliders;
pub mod components;
pub mod systems;

use systems::*;

use crate::game::resources::AppState;

// rotation in radians for player movement
pub const TOP_RIGHT: f32 = FRAC_PI_4;
pub const RIGHT: f32 = 0.0;
pub const BOTTOM_RIGHT: f32 = PI + FRAC_PI_2 + FRAC_PI_4;
pub const BOTTOM_LEFT: f32 = PI + FRAC_PI_4;
pub const LEFT: f32 = PI;
pub const TOP_LEFT: f32 = FRAC_PI_2 + FRAC_PI_4;

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
