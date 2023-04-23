use bevy::prelude::*;

use car_football::ball::BallPlugin;
use car_football::game::GamePlugin;
use car_football::physics::PhysicsPlugin;
use car_football::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}
