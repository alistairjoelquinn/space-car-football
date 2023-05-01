use bevy::prelude::*;

#[derive(Resource)]
pub struct GoalColorTimer {
    pub timer: Timer,
}

impl Default for GoalColorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
        }
    }
}
