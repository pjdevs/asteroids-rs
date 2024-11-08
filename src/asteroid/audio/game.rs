use crate::asteroid::game::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::utils::{spawn_music, spawn_random_sfx, spawn_sfx};

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidGameAudioAssets {
    #[asset(key = "gameplay.death.audio")]
    pub gameplay_death_audio: Handle<AudioSource>,

    #[asset(key = "gameplay.hit.audios", collection(typed))]
    pub gameplay_hit_audios: Vec<Handle<AudioSource>>,

    #[asset(key = "player.shoot.audios", collection(typed))]
    pub player_shoot_audios: Vec<Handle<AudioSource>>,

    #[asset(key = "gameplay.music.audio")]
    pub gameplay_music_audio: Handle<AudioSource>,
}

// Systems

pub fn audio_play_game_music_system(mut commands: Commands, assets: Res<AsteroidGameAudioAssets>) {
    spawn_music(&mut commands, assets.gameplay_music_audio.clone_weak());
}

pub fn audio_play_shoot_system(
    mut commands: Commands,
    mut events: EventReader<PlayerShoot>,
    assets: Res<AsteroidGameAudioAssets>,
) {
    for _ in events.read() {
        spawn_random_sfx(&mut commands, &assets.player_shoot_audios);
    }
}

pub fn audio_play_death_system(
    mut commands: Commands,
    query: Query<(), (Added<Dead>, With<AsteroidEnemy>)>,
    assets: Res<AsteroidGameAudioAssets>,
) {
    for _ in &query {
        spawn_sfx(&mut commands, assets.gameplay_death_audio.clone_weak());
    }
}

pub fn audio_play_hit_system(
    mut commands: Commands,
    query: Query<Ref<Health>, With<AsteroidEnemy>>,
    assets: Res<AsteroidGameAudioAssets>,
) {
    for health in &query {
        if health.is_changed() && !health.is_added() {
            spawn_random_sfx(&mut commands, &assets.gameplay_hit_audios);
        }
    }
}
