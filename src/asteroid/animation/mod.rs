use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;

use crate::asset;

#[derive(Deserialize, Asset, TypePath)]
pub struct Animation {
    pub play_mode: AnimationPlayMode,
    pub start: usize,
    pub end: usize,
    pub duration: f32,
}

impl Animation {
    #[inline(always)]
    fn frame_time(&self) -> f32 {
        self.duration / (self.end - self.start + 1) as f32
    }
}

#[derive(Deserialize)]
pub enum AnimationPlayMode {
    Loop,
    OneShot,
}

pub struct AsteroidAnimationPlugin;

impl Plugin for AsteroidAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationCompleted>()
            .add_systems(Update, animate);
    }
}

#[derive(Bundle, Default)]
pub struct AnimationBundle {
    pub animation: Handle<Animation>,
    pub player: AnimationPlayer,
}

#[derive(Component)]
pub struct AnimationPlayer {
    timer: Timer,
    started: bool,
    completed: bool,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            started: false,
            completed: false,
            timer: Timer::new(Duration::ZERO, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct AnimationCompleted {
    pub animated_entity: Entity,
}

fn animate(
    mut events: EventWriter<AnimationCompleted>,
    assets: Res<Assets<Animation>>,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Handle<Animation>,
        &mut TextureAtlas,
        &mut AnimationPlayer,
    )>,
) {
    query
        .iter_mut()
        .filter(|(_, _, _, animation)| !animation.completed)
        .for_each(|(entity, animation_asset, mut atlas, mut player)| {
            let animation = asset!(assets, animation_asset);

            if !player.started {
                player
                    .timer
                    .set_duration(Duration::from_secs_f32(animation.frame_time()));
                player.started = true;
            }

            player.timer.tick(time.delta());

            if player.timer.just_finished() {
                if atlas.index == animation.end {
                    match animation.play_mode {
                        AnimationPlayMode::Loop => {
                            atlas.index = animation.start;
                        }
                        AnimationPlayMode::OneShot => {
                            player.completed = true;

                            events.send(AnimationCompleted {
                                animated_entity: entity,
                            });
                        }
                    }
                } else {
                    atlas.index = atlas.index + 1;
                };
            }
        });
}

pub mod prelude {
    pub use super::*;
}
