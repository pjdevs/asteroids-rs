use super::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::{get, get_mut};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;

// TODO Split this in health, gameplay etc

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(
                OnEnter(AsteroidGameState::Game),
                (
                    gameplay_add_score_system,
                    gameplay_add_lives_system,
                    gameplay_spawn_background_system,
                ),
            )
            .add_systems(
                OnExit(AsteroidGameState::Game),
                (
                    remove_resource::<Score>,
                    remove_resource::<PlayerLives>,
                    despawn_entities_with::<PlayerRespawnTimer>,
                    despawn_entities_with::<Background>,
                ),
            )
            .add_systems(
                Update,
                (
                    (
                        gameplay_collision_damage_system.run_if(on_event::<CollisionEvent>()),
                        gameplay_death_system,
                    )
                        .chain()
                        .in_set(AsteroidGameplaySystem::UpdateDamageSystem),
                    gameplay_collision_despawn_system
                        .run_if(on_event::<CollisionEvent>())
                        .run_if(any_with_component::<KillCollision>),
                    ganeplay_setup_player_respawn.run_if(any_with_component::<AsteroidPlayer>),
                    gameplay_respawn_player.run_if(any_with_component::<PlayerRespawnTimer>),
                )
                    .run_if(in_state(AsteroidGameState::Game))
                    .in_set(AsteroidGameplaySystem::UpdateGameplay),
            )
            .add_systems(
                PostUpdate,
                (
                    gameplay_score_system,
                    gameplay_loose_lives,
                    gameplay_despawn_dead_system,
                )
                    .run_if(any_with_component::<Dead>)
                    .in_set(AsteroidGameplaySystem::PostUpdateGameplay),
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

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct DespawnIfDead;

#[derive(Component, Default)]
pub struct KillCollision;

pub trait Damager {
    fn get_damage(&self, health: &Health) -> i32;
}

#[derive(Component)]
pub struct CollisionDamager {
    damage_amount: i32,
}

impl Default for CollisionDamager {
    fn default() -> Self {
        Self { damage_amount: 10 }
    }
}

impl CollisionDamager {
    pub fn new(damage_amount: i32) -> Self {
        Self { damage_amount }
    }
}

impl Damager for CollisionDamager {
    fn get_damage(&self, _: &Health) -> i32 {
        self.damage_amount
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dead;

#[derive(Component)]
pub struct Health {
    max_health: i32,
    current_health: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100)
    }
}

impl Health {
    pub fn new(max_health: i32) -> Self {
        Self {
            max_health,
            current_health: max_health,
        }
    }

    #[inline(always)]
    pub fn get_max_health(&self) -> i32 {
        self.max_health
    }

    #[inline(always)]
    pub fn set_max_health(&mut self, new_max: i32) {
        self.max_health = new_max;
        self.current_health = self.max_health;
    }

    #[inline(always)]
    pub fn damage(&mut self, amount: i32) {
        self.current_health = (self.current_health - amount).clamp(0, self.max_health);
    }

    #[inline(always)]
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0
    }
}

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
    UpdateDamageSystem,
    PostUpdateGameplay,
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
    ));
}

fn gameplay_collision_despawn_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    query: Query<(), With<KillCollision>>,
) {
    for collision in collision_event.read() {
        handle_instant_kill(&mut commands, collision.first, &query);
        handle_instant_kill(&mut commands, collision.second, &query);
    }
}

fn handle_instant_kill(
    commands: &mut Commands,
    entity: Entity,
    query: &Query<(), With<KillCollision>>,
) {
    get!(_despawn, query, entity, return);
    commands.entity(entity).insert(Dead);
}

fn gameplay_collision_damage_system(
    mut collision_event: EventReader<CollisionEvent>,
    damager_query: Query<&CollisionDamager>,
    mut health_query: Query<&mut Health>,
) {
    for collision in collision_event.read() {
        handle_damage(
            collision.first,
            collision.second,
            &damager_query,
            &mut health_query,
        );
        handle_damage(
            collision.second,
            collision.first,
            &damager_query,
            &mut health_query,
        );
    }
}

fn handle_damage(
    first: Entity,
    second: Entity,
    damager_query: &Query<&CollisionDamager>,
    health_query: &mut Query<&mut Health>,
) {
    get!(damager, damager_query, first, return);
    get_mut!(health, health_query, second, return);

    let damage = damager.get_damage(&health);
    health.damage(damage);
}

fn gameplay_death_system(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), Without<Dead>>,
) {
    for (entity, health) in &health_query {
        if health.is_dead() {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn gameplay_despawn_dead_system(
    mut commands: Commands,
    dead_query: Query<Entity, (With<Dead>, With<DespawnIfDead>)>,
) {
    for entity in &dead_query {
        commands.entity(entity).despawn();
    }
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
                commands.spawn(PlayerRespawnTimer {
                    player_id: player.player_id,
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                });
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

fn ganeplay_setup_player_respawn(
    mut commands: Commands,
    mut player_lives: ResMut<PlayerLives>,
    query: Query<&AsteroidPlayer, Added<AsteroidPlayer>>,
) {
    for player in &query {
        if !player_lives.lives.contains_key(&player.player_id) {
            player_lives.lives.insert(player.player_id, 3);
        }
    }

    commands.trigger(PlayerLivesChanged);
}
