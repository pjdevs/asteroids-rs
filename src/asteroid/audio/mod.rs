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
use utils::Music;

// Plugin

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // Menu Loading
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::MainMenuLoading)
                .load_collection::<MainMenuAudioAssets>(),
        )
        // Game Loading
        .configure_loading_state(
            LoadingStateConfig::new(GameState::GameLoading).load_collection::<GameAudioAssets>(),
        )
        // Menu
        .add_systems(
            OnEnter(GameState::MainMenu),
            audio_play_menu_music_system.in_set(AudioSystem::MenuMusic),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                despawn_entities_with::<Music>,
                remove_resource::<MainMenuAudioAssets>,
            ),
        )
        .add_systems(
            Update,
            (audio_button_select_system, audio_button_click_system)
                .run_if(in_state(GameState::MainMenu))
                .in_set(AudioSystem::UpdateMenuSfx),
        )
        // Game
        .add_systems(
            OnEnter(GameState::Game),
            audio_play_game_music_system.in_set(AudioSystem::GameMusic),
        )
        .add_systems(
            OnExit(GameState::Game),
            (
                despawn_entities_with::<Music>,
                remove_resource::<GameAudioAssets>,
            ),
        )
        .add_systems(
            Update,
            (
                audio_play_shoot_system
                    .run_if(on_event::<ShootEvent>)
                    .after(PlayerSystem::UpdatePlayerActions),
                audio_play_hit_system.after(DamageSystem::FixedUpdateDamageSystem),
            )
                .run_if(in_state(GameState::Game))
                .in_set(AudioSystem::UpdateGameSfx),
        )
        .add_systems(
            PostUpdate,
            audio_play_death_system
                .run_if(any_with_component::<Dead>)
                .run_if(in_state(GameState::Game))
                .in_set(AudioSystem::PostUpdateGameSfx),
        );
    }
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AudioSystem {
    MenuMusic,
    GameMusic,
    UpdateGameSfx,
    UpdateMenuSfx,
    PostUpdateGameSfx,
}
