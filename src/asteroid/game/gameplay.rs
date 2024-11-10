use super::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::Collider;
use crate::asteroid::utils::prelude::*;
use crate::get_mut;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

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
                    gameplay_setup_observers.before(AsteroidPlayerSystem::OnEnterGameSpawn),
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
                ),
            )
            .add_systems(
                Update,
                (
                    gameplay_respawn_player.run_if(any_with_component::<PlayerRespawnTimer>),
                    gameplay_invincibility.run_if(any_with_component::<InvincibilityTimer>),
                )
                    .run_if(in_state(AsteroidGameState::Game))
                    .in_set(AsteroidGameplaySystem::UpdateGameplay),
            )
            .add_systems(
                PostUpdate,
                (gameplay_score_system, gameplay_loose_lives)
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

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InvincibilityTimer {
    times_flash: u64,
    duration_visible: f32,
    duration_invisible: f32,

    current_flashes: u64,
    is_visible: bool,
    timer: Timer,
}

impl InvincibilityTimer {
    pub fn new(times_flash: u64, duration_visible: f32, duration_invisible: f32) -> Self {
        Self {
            is_visible: true,
            times_flash,
            duration_visible,
            duration_invisible,
            timer: Timer::from_seconds(duration_visible, TimerMode::Once),
            current_flashes: 0,
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

fn gameplay_setup_observers(mut commands: Commands) {
    commands
        .observe(gameplay_setup_player_respawn)
        .insert(GameplayObserver);
}

fn gameplay_setup_player_respawn(
    trigger: Trigger<PlayerSpawned>,
    mut commands: Commands,
    mut player_lives: ResMut<PlayerLives>,
    mut query: Query<(&AsteroidPlayer, &mut Collider)>,
) {
    let player_entity = trigger.entity();
    get_mut!(components, query, player_entity, return);
    let (player, mut collider) = components;

    if !player_lives.lives.contains_key(&player.player_id) {
        player_lives.lives.insert(player.player_id, 3);
        commands.trigger(PlayerLivesChanged);
    }

    println!("SET INVINCIBLE");
    collider.enabled = false;
    commands
        .entity(player_entity)
        .insert(InvincibilityTimer::new(3, 0.5, 0.35));
}

fn gameplay_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut InvincibilityTimer, &mut Sprite, &mut Collider)>,
) {
    for (entity, mut invincibility, mut sprite, mut collider) in &mut query {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.just_finished() {
            invincibility.is_visible = !invincibility.is_visible;

            if invincibility.is_visible {
                invincibility.current_flashes += 1;
                let duration = invincibility.duration_visible;
                invincibility
                    .timer
                    .set_duration(Duration::from_secs_f32(duration));
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            } else {
                let duration = invincibility.duration_invisible;
                invincibility
                    .timer
                    .set_duration(Duration::from_secs_f32(duration));
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.1);
            }

            invincibility.timer.reset();

            if invincibility.current_flashes > invincibility.times_flash {
                collider.enabled = true;
                commands.entity(entity).remove::<InvincibilityTimer>();
            }
        }
    }
}
