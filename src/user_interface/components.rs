use bevy::prelude::*;

#[derive(Component)]
pub struct Loading {}

#[derive(Component)]
pub struct Menu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct Running {}

#[derive(Component)]
pub struct GameOver {}

#[derive(Component)]
pub struct Hud {}

#[derive(Component)]
pub struct Player1Score {
    pub value: u32,
}

#[derive(Component)]
pub struct Player2Score {
    pub value: u32,
}
