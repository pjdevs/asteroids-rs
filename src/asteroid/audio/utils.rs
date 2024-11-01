use bevy::prelude::*;

pub fn spawn_music(commands: &mut Commands, music: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source: music,
            settings: PlaybackSettings::LOOP,
        },
        Music,
    ));
}

pub fn spawn_sfx(commands: &mut Commands, sfx: Handle<AudioSource>) {
    commands.spawn(AudioBundle {
        source: sfx,
        settings: PlaybackSettings::DESPAWN,
    });
}

#[derive(Component)]
pub struct Music;
