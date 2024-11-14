use std::time::Duration;

use super::prelude::*;
use super::timed::{TimedAppExt, TimedEntityCommandsExt};
use crate::asteroid::animation::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::get_mut;
use bevy::prelude::*;

pub struct AsteroidEffectsPlugin;

impl Plugin for AsteroidEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_timed_component::<HitEffect>()
        .add_systems(
            OnExit(AsteroidGameState::Game),
            despawn_entities_with::<InvincibilityAnimation>,
        )
        .add_systems(
            Update,
            (
                // Hit Effect
                effect_start_hit,
                effect_play_hit.run_if(any_with_component::<HitEffect>),
                effect_stop_hit,

                // Invincibility Flash
                gameplay_start_invincibility_flash
                    .run_if(any_with_component::<Invincibility>),
                gameplay_stop_invincibility_flash,
                gameplay_update_invincibility_flash
                    .run_if(any_with_component::<InvincibilityFlash>),
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

// Hit Effect

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

// Invincibility Flash

#[derive(Component)]
struct InvincibilityAnimation;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InvincibilityFlash {
    duration_visible: f32,
    duration_invisible: f32,

    is_visible: bool,
    timer: Timer,
}

impl InvincibilityFlash {
    pub fn new(duration_visible: f32, duration_invisible: f32) -> Self {
        Self {
            is_visible: true,
            duration_visible,
            duration_invisible,
            timer: Timer::from_seconds(duration_visible, TimerMode::Once),
        }
    }
}

fn gameplay_start_invincibility_flash(
    mut commands: Commands,
    assets: Res<AsteroidPlayerAssets>,
    mut query: Query<Entity, Added<Invincibility>>,
) {
    for entity in &mut query {
        commands
            .entity(entity)
            .insert(InvincibilityFlash::new(0.5, 0.35))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: assets.player_invincible_texture.clone_weak(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(64.0)),
                            color: Color::srgb(1.0, 2.0, 2.0),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    TextureAtlas {
                        layout: assets.player_invincible_layout.clone_weak(),
                        index: 0,
                    },
                    Animation::new(AnimationPlayMode::Loop, 0, 12, 1.0),
                    InvincibilityAnimation,
                    #[cfg(feature = "dev")]
                    Name::new("Player Invincibility Animation"),
                ));
            });
    }
}

fn gameplay_stop_invincibility_flash(
    mut commands: Commands,
    mut removed: RemovedComponents<Invincibility>,
    mut query: Query<&mut Sprite>,
) {
    for entity in removed.read() {
        commands
            .entity(entity)
            .despawn_descendants()
            .remove::<InvincibilityFlash>();

        get_mut!(mut sprite, query, entity, continue);
        sprite.color = Color::WHITE;
    }
}

fn gameplay_update_invincibility_flash(
    time: Res<Time>,
    mut query: Query<(&mut InvincibilityFlash, &mut Sprite)>,
) {
    for (mut invincibility, mut sprite) in &mut query {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.just_finished() {
            invincibility.is_visible = !invincibility.is_visible;

            if invincibility.is_visible {
                let duration = invincibility.duration_visible;
                invincibility
                    .timer
                    .set_duration(Duration::from_secs_f32(duration));
                sprite.color = Color::WHITE;
            } else {
                let duration = invincibility.duration_invisible;
                invincibility
                    .timer
                    .set_duration(Duration::from_secs_f32(duration));
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.1);
            }

            invincibility.timer.reset();
        }
    }
}