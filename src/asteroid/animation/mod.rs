use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;

use crate::asset;

#[derive(Deserialize, Asset, TypePath)]
pub struct AnimationAsset {
    pub play_mode: AnimationPlayMode,
    pub start: usize,
    pub end: usize,
    pub duration: f32,
}

impl AnimationAsset {
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

#[derive(Component)]
pub struct Animation {
    animation: Handle<AnimationAsset>,
    timer: Timer,
    started: bool,
    completed: bool,
}

impl Animation {
    pub fn new(animation: Handle<AnimationAsset>) -> Self {
        Self {
            animation,
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
    assets: Res<Assets<AnimationAsset>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlas, &mut Animation)>,
) {
    query
        .iter_mut()
        .filter(|(_, _, animation)| !animation.completed)
        .for_each(|(entity, mut atlas, mut animation)| {
            let animation_asset = asset!(assets, &animation.animation);

            if !animation.started {
                animation.timer.set_duration(Duration::from_secs_f32(animation_asset.frame_time()));
                animation.started = true;
            }

            animation.timer.tick(time.delta());

            if animation.timer.just_finished() {
                if atlas.index == animation_asset.end {
                    match animation_asset.play_mode {
                        AnimationPlayMode::Loop => {
                            atlas.index = animation_asset.start;
                        }
                        AnimationPlayMode::OneShot => {
                            animation.completed = true;

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
