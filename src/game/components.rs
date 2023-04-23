use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RpgSpriteHandles {
    pub handles: Vec<HandleUntyped>,
}
