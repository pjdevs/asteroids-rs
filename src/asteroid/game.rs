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
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AsteroidGamePlugin;

// TODO Check how we can provide a state to plugins to run their systems in this state

impl Plugin for AsteroidGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Game Startup
            .add_systems(Startup, game_startup_system)
            // States
            .init_state::<AsteroidGameState>()
            .init_state::<AsteroidPauseState>()
            // Main Menu
            .add_systems(OnEnter(AsteroidGameState::MainMenu), ui_menu_setup_system)
            .add_systems(
                OnExit(AsteroidGameState::MainMenu),
                despawn_entities_with::<Node>,
            )
            // Game Loading
            .add_loading_state::<AsteroidGameState>(
                LoadingState::new(AsteroidGameState::GameLoadingScreen)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "asteroid.assets.ron",
                    )
                    .load_collection::<AsteroidPlayerAssets>()
                    .load_collection::<AsteroidEnemyAssets>()
                    .continue_to_state(AsteroidGameState::InGame),
            )
            // In Game
            .add_systems(
                OnEnter(AsteroidGameState::InGame),
                (spawn_first_player_system, ui_in_game_setup_system),
            )
            .add_systems(
                OnExit(AsteroidGameState::InGame),
                (
                    remove_resource::<AsteroidPlayerAssets>,
                    remove_resource::<AsteroidEnemyAssets>,
                    despawn_entities_with::<Node>,
                    despawn_entities_with::<AsteroidPlayer>,
                    despawn_entities_with::<AsteroidProjectile>,
                    despawn_entities_with::<AsteroidEnemy>,
                ),
            )
            .configure_sets(
                Update,
                (
                    AsteroidInputSystem::UpdateInput,
                    AsteroidPlayerSystem::UpdatePlayerActions,
                    AsteroidEnemySystem::UpdateSpawnEnemies,
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
