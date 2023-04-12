use bevy::prelude::*;

use car_football::game::systems::spawn_camera;
use car_football::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}
