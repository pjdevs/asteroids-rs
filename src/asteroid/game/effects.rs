use super::prelude::*;
use crate::asteroid::core::prelude::*;
use bevy::prelude::*;

pub struct AsteroidEffectsPlugin;

impl Plugin for AsteroidEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                effect_start_enemy_hit.run_if(any_with_component::<AsteroidEnemy>),
                effect_play_hit.run_if(any_with_component::<HitEffect>),
            )
                .run_if(in_state(AsteroidGameState::Game)),
        );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct HitEffect {
    timer: Timer,
    playing: bool,
}

fn effect_start_enemy_hit(mut commands: Commands, query: Query<(Entity, Ref<Health>)>) {
    for (entity, health) in &query {
        if health.is_changed() && !health.is_added() {
            commands.entity(entity).insert(HitEffect {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
                playing: false,
            });
        }
    }
}

fn effect_play_hit(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut HitEffect)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut hit) in &mut query {
        if !hit.playing {
            sprite.color = Color::srgb(3.0, 1.5, 0.0);
            hit.playing = true;
        }

        hit.timer.tick(time.delta());

        if hit.timer.just_finished() {
            hit.playing = false;
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
            commands.entity(entity).remove::<HitEffect>();
        }
    }
}
