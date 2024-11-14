use super::spawner::SpawnerAsset;
use crate::asteroid::animation::AnimationAsset;
use crate::asteroid::core::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_trauma_shake::Shake;

pub struct AsteroidSetupPlugin;

impl Plugin for AsteroidSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SizeAsset>::new(&["size.ron"]))
            .add_plugins(RonAssetPlugin::<SpawnerAsset>::new(&["spawner.ron"]))
            .add_plugins(RonAssetPlugin::<AnimationAsset>::new(&["anim.ron"]))
            // Game Startup
            .add_systems(Startup, game_startup_system)
            // States
            .init_state::<AsteroidGameState>()
            // Menu Loading State
            .add_loading_state(
                LoadingState::new(AsteroidGameState::MainMenuLoading)
                    .continue_to_state(AsteroidGameState::MainMenu)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("menu.assets.ron"),
            )
            // Game Loading State
            .add_loading_state(
                LoadingState::new(AsteroidGameState::GameLoading)
                    .continue_to_state(AsteroidGameState::Game)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron"),
            )
            // Utils
            .add_systems(
                Update,
                game_exit_system.run_if(
                    in_state(AsteroidGameState::Game)
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

fn game_exit_system(mut next_state: ResMut<NextState<AsteroidGameState>>) {
    next_state.set(AsteroidGameState::MainMenuLoading);
}
