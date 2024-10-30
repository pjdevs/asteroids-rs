use crate::asteroid::core::prelude::*;
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct AsteroidSetupPlugin;

impl Plugin for AsteroidSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SizeAsset>::new(&["size.ron"]))
            // Game Startup
            .add_systems(Startup, game_startup_system)
            // States
            .init_state::<AsteroidGameState>()
            // Game Loading State
            .add_loading_state(
                LoadingState::new(AsteroidGameState::GameLoadingScreen)
                    .continue_to_state(AsteroidGameState::InGame)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron"),
            )
            // Utils
            .add_systems(
                Update,
                game_exit_system.run_if(
                    in_state(AsteroidGameState::InGame)
                        .and_then(input_just_released(KeyCode::Escape)),
                ),
            );
    }
}

fn game_startup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_exit_system(mut next_state: ResMut<NextState<AsteroidGameState>>) {
    next_state.set(AsteroidGameState::MainMenu);
}
