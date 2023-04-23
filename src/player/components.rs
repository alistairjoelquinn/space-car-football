use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub id: u32,
    pub score: u32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Move,
}
