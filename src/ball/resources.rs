use bevy::prelude::*;

#[derive(Resource)]
pub struct BallTimer {
    pub timer: Timer,
}

impl Default for BallTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}
