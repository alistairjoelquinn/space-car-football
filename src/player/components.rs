use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: u32,
    pub score: u32,
}
