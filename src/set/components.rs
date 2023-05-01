use bevy::prelude::*;

#[derive(Component)]
pub struct Background {}

#[derive(Component)]
pub struct SpaceBarrier {}

#[derive(Component, Debug)]
pub struct Meteor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Goal {
    pub user_id: u32,
}
