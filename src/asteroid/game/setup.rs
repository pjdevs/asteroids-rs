use super::spawner::SpawnerAsset;
use crate::asteroid::animation::Animation;
use crate::asteroid::core::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_trauma_shake::Shake;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SizeAsset>::new(&["size.ron"]))
            .add_plugins(RonAssetPlugin::<SpawnerAsset>::new(&["spawner.ron"]))
            .add_plugins(RonAssetPlugin::<Animation>::new(&["anim.ron"]))
            // Game Startup
            .add_systems(Startup, game_startup_system)
            // States
            .init_state::<GameState>()
            // Menu Loading State
            .add_loading_state(
                LoadingState::new(GameState::MainMenuLoading)
                    .continue_to_state(GameState::MainMenu)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("menu.assets.ron"),
            )
            // Game Loading State
            .add_loading_state(
                LoadingState::new(GameState::GameLoading)
                    .continue_to_state(GameState::Game)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron"),
            )
            // Utils
            .add_systems(
                Update,
                game_exit_system.run_if(
                    in_state(GameState::Game)
                        .and_then(input_just_released(KeyCode::Escape)),
                ),
            );
    }
}

fn game_startup_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            intensity: 0.2,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 1.0,
            ..Default::default()
        },
        Shake::default(),
    ));
}

fn game_exit_system(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenuLoading);
}
