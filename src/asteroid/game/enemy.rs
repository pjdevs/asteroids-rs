use super::prelude::*;
use crate::asset;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::Rng;
use std::time::Duration;

pub struct AsteroidEnemyPlugin;

impl Plugin for AsteroidEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AsteroidGameState::Game),
            (
                remove_resource::<AsteroidEnemyAssets>,
                remove_resource::<AsteroidEnemySpawner>,
                despawn_entities_with::<AsteroidEnemy>,
            ),
        )
        .add_systems(
            Update,
            spawn_enemies_system
                .run_if(
                    in_state(AsteroidGameState::Game)
                        .and_then(on_spawn_timer::<AsteroidEnemySpawner>()),
                )
                .in_set(AsteroidEnemySystem::UpdateSpawnEnemies),
        )
        .configure_loading_state(
            LoadingStateConfig::new(AsteroidGameState::GameLoading)
                .load_collection::<AsteroidEnemyAssets>()
                .init_resource::<AsteroidEnemySpawner>(),
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidEnemyAssets {
    #[asset(key = "enemy.texture")]
    pub enemy_texture: Handle<Image>,

    #[asset(path = "enemy.size.ron")]
    pub enemy_size: Handle<SizeAsset>,

    #[asset(path = "enemy.spawner.ron")]
    pub enemy_spawner: Handle<SpawnerAsset>,
}

// Resources
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AsteroidEnemySpawner {
    pub enabled: bool,
    #[reflect(ignore)]
    pub spawner_asset: Handle<SpawnerAsset>,
}

impl Spawner for AsteroidEnemySpawner {
    fn spawner(&self) -> Handle<SpawnerAsset> {
        self.spawner_asset.clone_weak()
    }
}

impl FromWorld for AsteroidEnemySpawner {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<Res<AsteroidEnemyAssets>>::new(world);
        let enemy_assets = system_state.get(world);

        AsteroidEnemySpawner {
            enabled: true,
            spawner_asset: enemy_assets.enemy_spawner.clone_weak(),
        }
    }
}

// Components

#[derive(Component, Default)]
pub struct AsteroidEnemy;

#[derive(Bundle, Default)]
pub struct AsteroidEnemyBundle {
    enemy: AsteroidEnemy,
    sprite: SpriteBundle,
    movement: Movement,
    collider: Collider,
    layers: CollisionLayers,
    border: TunnelBorder,
    health: Health,
    damager: CollisionDamager,
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidEnemySystem {
    UpdateSpawnEnemies,
}

pub trait Spawner: Resource {
    fn spawner(&self) -> Handle<SpawnerAsset>;
}

fn on_spawn_timer<S: Spawner>(
) -> impl FnMut(Res<Time>, Res<S>, Res<Assets<SpawnerAsset>>) -> bool + Clone {
    let mut timer = Timer::new(Duration::ZERO, TimerMode::Repeating);

    move |time: Res<Time>, spawner: Res<S>, spawner_assets: Res<Assets<SpawnerAsset>>| {
        if spawner.is_changed() {
            let spawner_asset = asset!(spawner_assets, &spawner.spawner());
            timer.set_duration(Duration::from_millis(spawner_asset.spawn_delay_ms));
        }

        timer.tick(time.delta());
        timer.just_finished()
    }
}

fn spawn_enemies_system(
    mut commands: Commands,
    enemy_spawner: Res<AsteroidEnemySpawner>,
    enemy_assets: Res<AsteroidEnemyAssets>,
    size_assets: Res<Assets<SizeAsset>>,
    spawner_assets: Res<Assets<SpawnerAsset>>,
    camera_query: Query<&Camera>,
) {
    if !enemy_spawner.enabled {
        return;
    }

    let spawner_asset = asset!(spawner_assets, &enemy_spawner.spawner_asset);
    let size = asset!(size_assets, &enemy_assets.enemy_size);
    let camera = camera_query.single();
    let mut random = rand::thread_rng();

    let min_max_angle = spawner_asset.min_max_angle * std::f32::consts::PI;
    let random_angle = random.gen_range(min_max_angle.x..=min_max_angle.y);
    let random_speed =
        random.gen_range(spawner_asset.min_max_speed.x..=spawner_asset.min_max_speed.y);
    let random_velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let random_angular_velocity = random
        .gen_range(spawner_asset.min_max_angular_speed.x..=spawner_asset.min_max_angular_speed.y);
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position =
        2.0 * half_screen_size * Vec2::from(random.gen::<(f32, f32)>()).round() - half_screen_size;
    let random_scale =
        random.gen_range(spawner_asset.min_max_scale.x..=spawner_asset.min_max_scale.y);

    let enemy = AsteroidEnemyBundle {
        sprite: SpriteBundle {
            texture: enemy_assets.enemy_texture.clone(),
            sprite: Sprite {
                custom_size: Some(size.sprite_size * random_scale),
                ..Default::default()
            },
            ..Default::default()
        },
        movement: Movement {
            position: random_position,
            velocity: random_velocity,
            angular_velocity: random_angular_velocity,
            ..Default::default()
        },
        collider: Collider::from_shape(Shape::Circle(BoundingCircle::new(
            Vec2::ZERO,
            size.collider_size.x * random_scale / 2.0,
        ))),
        layers: CollisionLayers::new(layers::ENEMY_MASK, layers::PLAYER_MASK),
        damager: CollisionDamager::new(100),
        ..Default::default()
    };

    if cfg!(feature = "dev") {
        commands.spawn((enemy, Name::new("Enemy")));
    } else {
        commands.spawn(enemy);
    }
}
