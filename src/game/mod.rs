pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy::window::PresentMode;
use leafwing_input_manager::prelude::*;

use crate::game::resources::*;
use crate::game::systems::*;
use crate::player::components::Action;
use crate::set::systems::*;
use crate::user_interface::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_startup_system(spawn_camera)
            .init_resource::<GoalTimer>()
            .init_resource::<Score>()
            .insert_resource(GameAsset::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Car Football".into(),
                    resolution: (1200., 700.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
            .add_plugin(InputManagerPlugin::<Action>::default())
            // global app related systems
            .add_system(detect_user_key_input)
            .add_systems((
                // loading systems
                spawn_loading_screen.in_schedule(OnEnter(AppState::Loading)),
                despawn_loading_screen.in_schedule(OnExit(AppState::Loading)),
                load_assets.in_schedule(OnEnter(AppState::Loading)),
                check_assets.run_if(in_state(AppState::Loading)),
                // menu systems
                spawn_menu_screen.in_schedule(OnEnter(AppState::Menu)),
                play_menu_music.in_schedule(OnEnter(AppState::Menu)),
                despawn_menu_screen.in_schedule(OnExit(AppState::Menu)),
                click_play_button.run_if(in_state(AppState::Menu)),
                // game display systems
                spawn_game_screen.in_schedule(OnEnter(AppState::Running)),
                despawn_game_screen.in_schedule(OnExit(AppState::Running)),
                spawn_hud.in_schedule(OnEnter(AppState::Running)),
                despawn_hud.in_schedule(OnExit(AppState::Running)),
                // game over systems
                spawn_game_over_screen.in_schedule(OnEnter(AppState::GameOver)),
                despawn_game_over_screen
                    .in_schedule(OnExit(AppState::GameOver)),
            ))
            .add_systems((
                // game logic systems
                handle_goal_color.run_if(in_state(AppState::Running)),
                set_meteor_window_boundary.run_if(in_state(AppState::Running)),
                handle_collision_sounds.run_if(in_state(AppState::Running)),
                handle_user_goal.run_if(in_state(AppState::Running)),
                update_score_text.run_if(in_state(AppState::Running)),
            ));
    }
}
