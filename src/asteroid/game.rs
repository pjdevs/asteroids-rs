use super::assets::SizeAsset;
use super::gameplay::{
    gameplay_cleanup, gameplay_setup, AsteroidGameplayAssets, AsteroidGameplaySystem,
};
use super::states::{AsteroidGameState, AsteroidPauseState};
use crate::asteroid::enemy::{AsteroidEnemy, AsteroidEnemyAssets, AsteroidEnemySystem};
use crate::asteroid::input::AsteroidInputSystem;
use crate::asteroid::physics::AsteroidPhysicsSystem;
use crate::asteroid::player::{
    spawn_first_player_system, AsteroidPlayer, AsteroidPlayerAssets, AsteroidPlayerSystem,
};
use crate::asteroid::projectile::AsteroidProjectile;
use crate::asteroid::systems::{despawn_entities_with, remove_resource};
use crate::asteroid::ui::{ui_in_game_setup_system, ui_menu_setup_system};
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct AsteroidGamePlugin;

// TODO Check how we can provide a state to plugins to run their systems in this state

impl Plugin for AsteroidGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SizeAsset>::new(&["size.ron"]))
            // Game Startup
            .add_systems(Startup, game_startup_system)
            // States
            .init_state::<AsteroidGameState>()
            // .init_state::<AsteroidPauseState>()
            // Main Menu
            .add_systems(OnEnter(AsteroidGameState::MainMenu), ui_menu_setup_system)
            .add_systems(
                OnExit(AsteroidGameState::MainMenu),
                despawn_entities_with::<Node>,
            )
            // Game Loading
            .add_loading_state::<AsteroidGameState>(
                LoadingState::new(AsteroidGameState::GameLoadingScreen)
                    .continue_to_state(AsteroidGameState::InGame)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "game.assets.ron",
                    )
                    .load_collection::<AsteroidPlayerAssets>()
                    .load_collection::<AsteroidEnemyAssets>()
                    .load_collection::<AsteroidGameplayAssets>(),
            )
            // In Game
            .add_systems(
                OnEnter(AsteroidGameState::InGame),
                (
                    spawn_first_player_system,
                    ui_in_game_setup_system,
                    gameplay_setup,
                ),
            )
            .add_systems(
                OnExit(AsteroidGameState::InGame),
                (
                    remove_resource::<AsteroidPlayerAssets>,
                    remove_resource::<AsteroidEnemyAssets>,
                    remove_resource::<AsteroidGameplayAssets>,
                    despawn_entities_with::<Node>,
                    despawn_entities_with::<AsteroidPlayer>,
                    despawn_entities_with::<AsteroidProjectile>,
                    despawn_entities_with::<AsteroidEnemy>,
                    gameplay_cleanup,
                ),
            )
            .add_systems(
                Update,
                game_exit.run_if(
                    in_state(AsteroidGameState::InGame)
                        .and_then(input_just_released(KeyCode::Escape)),
                ),
            )
            .configure_sets(
                Update,
                (
                    AsteroidInputSystem::UpdateInput,
                    AsteroidPlayerSystem::UpdatePlayerActions,
                    AsteroidEnemySystem::UpdateSpawnEnemies,
                    AsteroidGameplaySystem::UpdateGameplay,
                )
                    .run_if(in_state(AsteroidGameState::InGame)),
            )
            .configure_sets(
                FixedUpdate,
                AsteroidPhysicsSystem::FixedUpdateMovement
                    .run_if(in_state(AsteroidGameState::InGame)),
            )
            .configure_sets(
                PostUpdate,
                AsteroidPhysicsSystem::PostUpdateExtrapolateTransform
                    .run_if(in_state(AsteroidGameState::InGame)),
            );
    }
}

fn game_startup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_exit(mut next_state: ResMut<NextState<AsteroidGameState>>) {
    next_state.set(AsteroidGameState::MainMenu);
}
