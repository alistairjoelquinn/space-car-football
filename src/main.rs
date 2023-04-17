use bevy::prelude::*;
use bevy::window::PresentMode;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use car_football::ball::BallPlugin;
use car_football::game::systems::spawn_camera;
use car_football::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GREEN))
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(spawn_camera)
        .run();
}
