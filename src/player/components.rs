use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub id: u32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Move,
}
