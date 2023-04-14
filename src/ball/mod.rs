use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball);
    }
}
