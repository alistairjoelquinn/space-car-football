pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::ball::resources::BallTimer;
use crate::ball::systems::*;
use crate::game::resources::AppState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallTimer>()
            .add_system(spawn_ball.in_schedule(OnExit(AppState::Menu)))
            .add_system(ball_goal_timer.run_if(in_state(AppState::Running)));
    }
}
