use super::prelude::*;
use super::timed::{TimedAppExt, TimedEntityCommandsExt};
use crate::asteroid::animation::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::{asset, get, get_mut};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_trauma_shake::prelude::*;
use std::time::Duration;

// TODO Split this in another module
// TODO Expose effects values?

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_timed_component::<HitEffect>()
            .add_systems(
                OnExit(GameState::Game), despawn_entities_with::<Effect>,
            )
            .add_systems(
                FixedUpdate,
                (effect_explode_enemy, effect_despawn_exploded_enemy)
                    .after(DamageSystem::FixedUpdateDamageSystem)
                    .run_if(in_state(GameState::Game))
                    .in_set(EffectsSystem::FixedUpdateEffects),
            )
            .add_systems(
                Update,
                (
                    // Hit Effect
                    effect_start_hit,
                    effect_play_hit.run_if(any_with_component::<HitEffect>),
                    effect_stop_hit,
                    // Invincibility Flash
                    effect_start_invincibility_flash.run_if(any_with_component::<Invincibility>),
                    effect_stop_invincibility_flash,
                    effect_update_invincibility_flash
                        .run_if(any_with_component::<InvincibilityFlash>),
                )
                    .chain()
                    .run_if(in_state(GameState::Game))
                    .in_set(EffectsSystem::UpdateEffects),
            )
            .configure_loading_state(
                LoadingStateConfig::new(GameState::GameLoading).load_collection::<EffectsAssets>(),
            );
    }
}

#[derive(Resource, AssetCollection)]
pub struct EffectsAssets {
    #[asset(key = "player.invincible.texture")]
    pub player_invincible_texture: Handle<Image>,

    #[asset(key = "player.invincible.layout")]
    pub player_invincible_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "animations/player_invincibility.anim.ron")]
    pub player_invincibility_animation: Handle<Animation>,

    #[asset(path = "animations/enemy_explosion.anim.ron")]
    pub enemy_explosion_animation: Handle<Animation>,
}

#[derive(Component)]
#[require(
    Sprite,
    AnimationPlayer,
    Transform(|| Transform::from_xyz(0.0, 0.0, 1.0))
)]
struct Effect;

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum EffectsSystem {
    UpdateEffects,
    FixedUpdateEffects,
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

fn effect_start_invincibility_flash(
    mut commands: Commands,
    effect_assets: Res<EffectsAssets>,
    size_assets: Res<Assets<SizeAsset>>,
    player_assets: Res<PlayerAssets>,
    mut query: Query<Entity, Added<Invincibility>>,
) {
    let player_size = asset!(size_assets, &player_assets.player_size);

    for entity in &mut query {
        commands
            .entity(entity)
            .insert(InvincibilityFlash::new(0.5, 0.35))
            .with_children(|parent| {
                parent.spawn((
                    Effect,
                    Sprite {
                        image: effect_assets.player_invincible_texture.clone_weak(),
                        texture_atlas: Some(effect_assets.player_invincible_layout.clone_weak().into()),
                        color: Color::srgb(1.0, 2.0, 2.0),
                        custom_size: Some(player_size.sprite_size),
                        ..Default::default()
                    },
                    AnimationPlayer::new(effect_assets.player_invincibility_animation.clone_weak()),
                    #[cfg(feature = "dev")]
                    Name::new("Player Invincibility Animation"),
                ));
            });
    }
}

fn effect_stop_invincibility_flash(
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

fn effect_update_invincibility_flash(
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

// Enemy explosion

#[derive(Component)]
struct EnemyExplosion;

// TODO Re-enable Trauma by migrating to 0.15
fn effect_explode_enemy(
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
    effects_assets: Res<EffectsAssets>,
    size_assets: Res<Assets<SizeAsset>>,
    mut query: Query<(&Movement, &Scaled), (With<Enemy>, Added<Dead>)>,
    // mut shake_query: Query<&mut Shake>,
) {
    // let mut shake = shake_query.single_mut();

    for (movement, scaled) in &mut query {
        // shake.add_trauma(0.2 * scaled.scale);

        let enemy_size = asset!(size_assets, &enemy_assets.enemy_size);

        commands.spawn((
            Effect,
            Sprite {
                image: enemy_assets.enemy_texture.clone_weak(),
                texture_atlas: Some(enemy_assets.enemy_layout.clone_weak().into()),
                color: Color::srgb(5.0, 3.0, 0.0),
                custom_size: Some(enemy_size.sprite_size),
                ..Default::default()
            },
            AnimationPlayer::new(effects_assets.enemy_explosion_animation.clone_weak()),
            Movement {
                position: movement.position,
                rotation: movement.rotation,
                velocity: movement.velocity,
                ..Default::default()
            },
            *scaled,
            EnemyExplosion,
        ));
    }
}

fn effect_despawn_exploded_enemy(
    mut commands: Commands,
    mut events: EventReader<AnimationCompleted>,
    query: Query<(), With<EnemyExplosion>>,
) {
    for event in events.read() {
        get!(_enemy_explosion, query, event.animated_entity, continue);
        commands.entity(event.animated_entity).despawn();
    }
}
