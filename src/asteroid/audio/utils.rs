use bevy::prelude::*;
use rand::seq::SliceRandom;

pub fn spawn_music(commands: &mut Commands, music: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source: music,
            settings: PlaybackSettings::LOOP,
        },
        Music,
        #[cfg(feature = "dev")]
        Name::new("Music"),
    ));
}

pub fn spawn_sfx(commands: &mut Commands, sfx: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source: sfx,
            settings: PlaybackSettings::DESPAWN,
        },
        #[cfg(feature = "dev")]
        Name::new("SFX"),
    ));
}

pub fn spawn_random_sfx(commands: &mut Commands, sfxs: &Vec<Handle<AudioSource>>) {
    let mut rng = rand::thread_rng();
    let sfx = sfxs.choose(&mut rng).expect("Cannot find random sfx");

    spawn_sfx(commands, sfx.clone_weak());
}

#[derive(Component)]
pub struct Music;
