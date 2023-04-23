use bevy::prelude::*;

use space_car_football::ball::BallPlugin;
use space_car_football::game::GamePlugin;
use space_car_football::physics::PhysicsPlugin;
use space_car_football::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}
