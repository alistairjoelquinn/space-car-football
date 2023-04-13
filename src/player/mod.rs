use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

pub mod components;
pub mod systems;

use systems::*;

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
        app.add_startup_system(spawn_players)
            .add_system(player_movement)
            .add_system(set_window_boundary);
    }
}
