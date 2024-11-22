use crate::asteroid::ui::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::utils::{spawn_music, spawn_sfx};

// Assets

#[derive(Resource, AssetCollection)]
pub struct MainMenuAudioAssets {
    #[asset(key = "menu.music.audio")]
    pub menu_music_audio: Handle<AudioSource>,

    #[asset(key = "menu.button.select")]
    pub menu_button_select_audio: Handle<AudioSource>,

    #[asset(key = "menu.button.start")]
    pub menu_button_start_audio: Handle<AudioSource>,
}

// Systems

pub fn audio_play_menu_music_system(mut commands: Commands, assets: Res<MainMenuAudioAssets>) {
    spawn_music(&mut commands, assets.menu_music_audio.clone_weak());
}

pub fn audio_button_select_system(
    mut commands: Commands,
    assets: Res<MainMenuAudioAssets>,
    mut query: Query<&Interaction, Changed<Interaction>>,
) {
    for interaction in &mut query {
        if *interaction == Interaction::Hovered {
            spawn_sfx(&mut commands, assets.menu_button_select_audio.clone_weak());
        }
    }
}

pub fn audio_button_click_system(
    mut commands: Commands,
    assets: Res<MainMenuAudioAssets>,
    mut events: EventReader<ButtonEvent>,
) {
    // for now there is only clicked event
    for _ in events.read() {
        spawn_sfx(&mut commands, assets.menu_button_start_audio.clone_weak());
    }
}
