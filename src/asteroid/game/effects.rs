use super::prelude::*;
use super::timed::{TimedAppExt, TimedEntityCommandsExt};
use crate::asteroid::core::prelude::*;
use crate::get_mut;
use bevy::prelude::*;

pub struct AsteroidEffectsPlugin;

impl Plugin for AsteroidEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_timed_component::<HitEffect>().add_systems(
            Update,
            (
                effect_start_hit,
                effect_play_hit.run_if(any_with_component::<HitEffect>),
                effect_stop_hit,
            )
                .chain()
                .run_if(in_state(AsteroidGameState::Game))
                .in_set(AsteroidEffectsSystem::UpdateEffects),
        );
    }
}

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidEffectsSystem {
    UpdateEffects,
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct HitEffect;

fn effect_start_hit(
    mut commands: Commands,
    query: Query<(Entity, Ref<Health>), Without<Invincibility>>,
) {
    for (entity, health) in &query {
        if health.is_changed() && !health.is_added() && !health.is_dead() {
            commands.entity(entity).insert_timed(HitEffect, 0.1);
        }
    }
}

fn effect_play_hit(mut query: Query<&mut Sprite, Added<HitEffect>>) {
    for mut sprite in &mut query {
        sprite.color = Color::srgb(3.5, 2.5, 0.0);
    }
}

fn effect_stop_hit(mut removed: RemovedComponents<HitEffect>, mut query: Query<&mut Sprite>) {
    for entity in removed.read() {
        get_mut!(mut sprite, query, entity, continue);
        sprite.color = Color::WHITE;
    }
}
