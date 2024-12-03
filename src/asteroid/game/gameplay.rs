use super::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::input::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::get;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .register_timed_component::<Invincibility>()
            .add_systems(
                OnEnter(GameState::Game),
                (
                    gameplay_add_score_system,
                    gameplay_add_lives_system,
                    gameplay_spawn_background_system,
                    gameplay_setup_observers,
                    spawn_first_player_system.after(gameplay_setup_observers),
                    spawn_second_player_system
                        .after(gameplay_setup_observers)
                        .run_if(gamepad_connected()),
                ),
            )
            .add_systems(
                OnExit(GameState::Game),
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
                    spawn_second_player_system
                        .run_if(on_gamepad_connection())
                        .run_if(not(player_exists(2))),
                )
                    .in_set(GameplaySystem::UpdateGameplay)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                FixedPostUpdate,
                (gameplay_score_system, gameplay_loose_lives)
                    .run_if(any_with_component::<Dead>)
                    .before(DamageSystem::FixedPostUpdateDeathSystem)
                    .in_set(GameplaySystem::FixedPostUpdateGameplay),
            )
            .configure_loading_state(
                LoadingStateConfig::new(GameState::GameLoading).load_collection::<GameplayAssets>(),
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
pub struct PlayerRespawnTimer {
    player_id: u64,
    timer: Timer,
}

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
pub struct GameplayAssets {
    #[asset(key = "gameplay.background.texture")]
    pub background_texture: Handle<Image>,
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum GameplaySystem {
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
    query: Query<&Scaled, (With<Enemy>, Added<Dead>)>,
) {
    for scaled in &query {
        score.score += (10.0 * scaled.scale) as u64;

        commands.trigger(ScoreChanged);
    }
}

fn gameplay_spawn_background_system(
    mut commands: Commands,
    assets: Res<GameplayAssets>,
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
    query: Query<&Player, Added<Dead>>,
    mut next_state: ResMut<NextState<GameState>>,
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
        next_state.set(GameState::MainMenuLoading);
    }
}

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

// TODO Expose values?

fn gameplay_setup_player_respawn(
    trigger: Trigger<PlayerSpawned>,
    mut commands: Commands,
    mut player_lives: ResMut<PlayerLives>,
    query: Query<&Player>,
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
