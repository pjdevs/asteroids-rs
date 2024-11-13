use super::prelude::*;
use super::timed::TimedAppExt;
use crate::asteroid::animation::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::game::timed::*;
use crate::asteroid::input::prelude::*;
use crate::asteroid::physics::prelude::Collider;
use crate::asteroid::utils::prelude::*;
use crate::{get, get_mut};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .register_timed_component::<Invincibility>()
            .add_systems(
                OnEnter(AsteroidGameState::Game),
                (
                    gameplay_add_score_system,
                    gameplay_add_lives_system,
                    gameplay_spawn_background_system,
                    gameplay_setup_observers,
                    spawn_first_player_system.after(gameplay_setup_observers),
                    spawn_second_player_system
                        .after(gameplay_setup_observers)
                        .run_if(gamepad_connected(0)),
                ),
            )
            .add_systems(
                OnExit(AsteroidGameState::Game),
                (
                    remove_resource::<Score>,
                    remove_resource::<PlayerLives>,
                    despawn_entities_with::<PlayerRespawnTimer>,
                    despawn_entities_with::<Background>,
                    despawn_entities_with::<GameplayObserver>,
                    despawn_entities_with::<InvincibilityAnimation>,
                ),
            )
            .add_systems(
                Update,
                (
                    gameplay_respawn_player.run_if(any_with_component::<PlayerRespawnTimer>),
                    gameplay_start_invincibility
                        .after(gameplay_respawn_player)
                        .run_if(any_with_component::<Invincibility>),
                    gameplay_stop_invincibility,
                    gameplay_update_invincibility_flash
                        .run_if(any_with_component::<InvincibilityFlash>),
                )
                    .run_if(in_state(AsteroidGameState::Game))
                    .in_set(AsteroidGameplaySystem::UpdateGameplay),
            )
            .add_systems(
                FixedPostUpdate,
                (gameplay_score_system, gameplay_loose_lives)
                    .before(AsteroidDamageSystem::FixedPostUpdateDeathSystem)
                    .run_if(any_with_component::<Dead>)
                    .in_set(AsteroidGameplaySystem::FixedPostUpdateGameplay),
            )
            .configure_loading_state(
                LoadingStateConfig::new(AsteroidGameState::GameLoading)
                    .load_collection::<AsteroidGameplayAssets>(),
            );
    }
}

// Resources

#[derive(Resource, Default)]
pub struct Score {
    score: u64,
}

impl Score {
    #[inline]
    pub fn get_score(&self) -> u64 {
        self.score
    }
}

#[derive(Component)]
pub struct PlayerRespawnTimer {
    player_id: u64,
    timer: Timer,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InvincibilityFlash {
    duration_visible: f32,
    duration_invisible: f32,

    is_visible: bool,
    timer: Timer,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Invincibility;

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

#[derive(Resource, Default)]
pub struct PlayerLives {
    lives: HashMap<u64, u64>,
}

impl PlayerLives {
    #[inline]
    pub fn get_lives(&self) -> &HashMap<u64, u64> {
        &self.lives
    }
}

// Components

#[derive(Component)]
struct Background;

#[derive(Component)]
struct InvincibilityAnimation;

#[derive(Component)]
struct GameplayObserver;

// Events

#[derive(Event)]
pub struct PlayerLivesChanged;

#[derive(Event)]
pub struct ScoreChanged;

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidGameplayAssets {
    #[asset(key = "gameplay.background.texture")]
    pub background_texture: Handle<Image>,
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidGameplaySystem {
    UpdateGameplay,
    FixedPostUpdateGameplay,
}

fn gameplay_add_score_system(mut commands: Commands) {
    commands.init_resource::<Score>();
}

fn gameplay_add_lives_system(mut commands: Commands) {
    commands.init_resource::<PlayerLives>();
}

fn gameplay_score_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    query: Query<&AsteroidScaled, (With<AsteroidEnemy>, Added<Dead>)>,
) {
    for scaled in &query {
        score.score += (10.0 * scaled.scale) as u64;

        commands.trigger(ScoreChanged);
    }
}

fn gameplay_spawn_background_system(
    mut commands: Commands,
    assets: Res<AsteroidGameplayAssets>,
    camera_query: Query<&Camera>,
) {
    commands.spawn((
        SpriteBundle {
            texture: assets.background_texture.clone(),
            sprite: Sprite {
                custom_size: camera_query.single().logical_viewport_size(),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        },
        Background,
        #[cfg(feature = "dev")]
        Name::new("Background"),
    ));
}

fn gameplay_loose_lives(
    mut commands: Commands,
    mut player_lives: ResMut<PlayerLives>,
    query: Query<&AsteroidPlayer, Added<Dead>>,
    mut next_state: ResMut<NextState<AsteroidGameState>>,
) {
    for player in &query {
        if let Some(lives) = player_lives.lives.get_mut(&player.player_id) {
            *lives -= 1;

            commands.trigger(PlayerLivesChanged);

            if *lives > 0 {
                commands.spawn((
                    PlayerRespawnTimer {
                        player_id: player.player_id,
                        timer: Timer::from_seconds(1.0, TimerMode::Once),
                    },
                    #[cfg(feature = "dev")]
                    Name::new("Player Invincibility Timer"),
                ));
            }
        }
    }

    if player_lives.lives.values().sum::<u64>() <= 0 {
        next_state.set(AsteroidGameState::MainMenuLoading);
    }
}

// TODO Explose default value in asset for both

fn gameplay_respawn_player(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PlayerRespawnTimer)>,
) {
    for (entity, mut respawner) in &mut query {
        respawner.timer.tick(time.delta());

        if respawner.timer.just_finished() {
            let player_id = respawner.player_id;

            commands.add(SpawnPlayer::new(player_id));
            commands.entity(entity).despawn();
        }
    }
}

fn gameplay_setup_observers(mut commands: Commands) {
    commands.observe(gameplay_setup_player_respawn).insert((
        GameplayObserver,
        #[cfg(feature = "dev")]
        Name::new("Respawn Player Observer"),
    ));
}

fn gameplay_setup_player_respawn(
    trigger: Trigger<PlayerSpawned>,
    mut commands: Commands,
    mut player_lives: ResMut<PlayerLives>,
    query: Query<&AsteroidPlayer>,
) {
    let player_entity = trigger.entity();
    get!(player, query, player_entity, return);

    if !player_lives.lives.contains_key(&player.player_id) {
        player_lives.lives.insert(player.player_id, 3);
        commands.trigger(PlayerLivesChanged);
    }

    commands
        .entity(player_entity)
        .insert_timed(Invincibility, 3.0);
}

fn gameplay_start_invincibility(
    mut commands: Commands,
    assets: Res<AsteroidPlayerAssets>,
    mut query: Query<(Entity, &mut Collider), Added<Invincibility>>,
) {
    for (entity, mut collider) in &mut query {
        collider.enabled = false;

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

fn gameplay_stop_invincibility(
    mut commands: Commands,
    mut removed: RemovedComponents<Invincibility>,
    mut query: Query<(&mut Sprite, &mut Collider)>,
) {
    for entity in removed.read() {
        // TODO Make all of this less hacky
        commands
            .entity(entity)
            .despawn_descendants()
            .remove::<InvincibilityFlash>();

        get_mut!((mut sprite, mut collider), query, entity, continue);
        collider.enabled = true;
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
