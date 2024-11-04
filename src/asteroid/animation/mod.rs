use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    start: usize,
    end: usize,
    timer: Timer,
}

impl Animation {
    pub fn new(mode: TimerMode, start: usize, end: usize, fps: f32) -> Self {
        Self {
            start,
            end,
            timer: Timer::new(Duration::from_secs_f32(1.0 / fps), mode),
        }
    }
}

#[derive(Event)]
pub struct AnimationCompleted(Entity);

fn animate(
    mut events: EventWriter<AnimationCompleted>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlas, &mut Animation)>,
) {
    for (entity, mut atlas, mut animation) in &mut query {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if atlas.index == animation.end {
                atlas.index = animation.start;
                events.send(AnimationCompleted(entity));
            } else {
                atlas.index = atlas.index + 1;
            };
        }
    }
}
