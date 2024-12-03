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

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationCompleted>()
            .add_systems(Update, animate);
    }
}

#[derive(Component)]
#[require(Sprite)]
pub struct AnimationPlayer {
    animation: Handle<Animation>,
    timer: Timer,
    started: bool,
    completed: bool,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            animation: Default::default(),
            started: false,
            completed: false,
            timer: Timer::new(Duration::ZERO, TimerMode::Repeating),
        }
    }
}

impl AnimationPlayer {
    pub fn new(animation: Handle<Animation>) -> Self {
        Self {
            animation,
            ..Default::default()
        }
    }

    pub fn set_animation(&mut self, animation: Handle<Animation>) {
        self.animation = animation;
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
        &mut Sprite,
        &mut AnimationPlayer,
    )>,
) {
    query
        .iter_mut()
        .filter(|(_, _, animation)| !animation.completed)
        .for_each(|(entity, mut sprite, mut player)| {
            let Some(atlas) = sprite.texture_atlas.as_mut() else {
                return;
            };
            let animation = asset!(assets, &player.animation);
            
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
