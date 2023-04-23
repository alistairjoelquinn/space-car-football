use bevy::prelude::*;
use bevy::window::PresentMode;

use car_football::ball::BallPlugin;
use car_football::game::resources::*;
use car_football::game::systems::*;
use car_football::physics::PhysicsPlugin;
use car_football::player::systems::spawn_players;
use car_football::player::PlayerPlugin;

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .insert_resource(GameAsset::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Car Football".into(),
                resolution: (1200., 700.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_set.before(spawn_players))
        .add_systems((
            load_assets.in_schedule(OnEnter(AppState::Loading)),
            check_assets.run_if(in_state(AppState::Loading)),
        ))
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}
