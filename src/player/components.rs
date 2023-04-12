use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub id: u32,
    pub score: u32,
}
