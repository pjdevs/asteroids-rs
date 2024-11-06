use bevy::prelude::*;

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
    play_mode: AnimationPlayMode,
    start: usize,
    end: usize,
    timer: Timer,
    completed: bool,
}

impl Animation {
    pub fn new(play_mode: AnimationPlayMode, start: usize, end: usize, duration_secs: f32) -> Self {
        Self {
            play_mode,
            start,
            end,
            timer: Timer::from_seconds(
                duration_secs / (end - start + 1) as f32,
                TimerMode::Repeating,
            ),
            completed: false,
        }
    }
}

#[derive(Event)]
pub struct AnimationCompleted {
    pub animated_entity: Entity,
}

fn animate(
    mut events: EventWriter<AnimationCompleted>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlas, &mut Animation)>,
) {
    query.iter_mut().filter(|(_, _, a)| !a.completed).for_each(
        |(entity, mut atlas, mut animation)| {
            animation.timer.tick(time.delta());

            if animation.timer.just_finished() {
                if atlas.index == animation.end {
                    match animation.play_mode {
                        AnimationPlayMode::Loop => {
                            atlas.index = animation.start;
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
        },
    );
}

pub mod prelude {
    pub use super::*;
}
