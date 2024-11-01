mod game;
mod menu;
mod utils;

use super::core::prelude::*;
use super::game::prelude::*;
use super::utils::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use game::*;
use menu::*;

// Plugin

pub struct AsteroidAudioPlugin;

impl Plugin for AsteroidAudioPlugin {
    fn build(&self, app: &mut App) {
        // Menu Loading
        app.configure_loading_state(
            LoadingStateConfig::new(AsteroidGameState::MainMenuLoading)
                .load_collection::<AsteroidMainMenuAudioAssets>(),
        )
        // Game Loading
        .configure_loading_state(
            LoadingStateConfig::new(AsteroidGameState::GameLoading)
                .load_collection::<AsteroidGameAudioAssets>(),
        )
        // Menu
        .add_systems(
            OnEnter(AsteroidGameState::MainMenu),
            audio_play_menu_music_system.in_set(AsteroidAudioSystem::MenuMusic),
        )
        .add_systems(
            OnExit(AsteroidGameState::MainMenu),
            (
                despawn_entities_with::<Handle<AudioSource>>,
                remove_resource::<AsteroidMainMenuAudioAssets>,
            ),
        )
        .add_systems(
            Update,
            (audio_button_select_system, audio_button_click_system)
                .run_if(in_state(AsteroidGameState::MainMenu))
                .in_set(AsteroidAudioSystem::UpdateMenuSfx),
        )
        // Game
        .add_systems(
            OnEnter(AsteroidGameState::Game),
            audio_play_game_music_system.in_set(AsteroidAudioSystem::GameMusic),
        )
        .add_systems(
            OnExit(AsteroidGameState::Game),
            (
                despawn_entities_with::<Handle<AudioSource>>,
                remove_resource::<AsteroidGameAudioAssets>,
            ),
        )
        .add_systems(
            Update,
            (
                audio_play_shoot_system
                    .run_if(on_event::<PlayerShoot>())
                    .after(AsteroidPlayerSystem::UpdatePlayerActions),
                (
                    audio_play_death_system.run_if(any_with_component::<Dead>),
                    audio_play_hit_system,
                )
                    .after(AsteroidGameplaySystem::UpdateDamageSystem),
            )
                .run_if(in_state(AsteroidGameState::Game))
                .in_set(AsteroidAudioSystem::UpdateGameSfx),
        );
    }
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidAudioSystem {
    MenuMusic,
    GameMusic,
    UpdateGameSfx,
    UpdateMenuSfx,
}
