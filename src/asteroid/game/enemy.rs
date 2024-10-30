use super::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_asset_loader::prelude::*;
use std::time::Duration;

pub struct AsteroidEnemyPlugin {
    pub enemy_spawn_delay_seconds: u64,
}

impl Plugin for AsteroidEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AsteroidGameState::InGame),
            (
                remove_resource::<AsteroidEnemyAssets>,
                despawn_entities_with::<AsteroidEnemy>,
            ),
        )
        .add_systems(
            Update,
            spawn_enemies_system
                .run_if(in_state(AsteroidGameState::InGame))
                .run_if(on_timer(Duration::from_secs(
                    self.enemy_spawn_delay_seconds,
                )))
                .in_set(AsteroidEnemySystem::UpdateSpawnEnemies),
        )
        .configure_loading_state(
            LoadingStateConfig::new(AsteroidGameState::GameLoadingScreen)
                .load_collection::<AsteroidEnemyAssets>(),
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidEnemyAssets {
    #[asset(path = "sprites/asteroid01.png")]
    pub enemy_texture: Handle<Image>,

    #[asset(path = "enemy.size.ron")]
    pub enemy_size: Handle<SizeAsset>,
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

// TODO Expose min max speed angle etc

fn spawn_enemies_system(
    mut commands: Commands,
    enemy_assets: Res<AsteroidEnemyAssets>,
    size_assets: Res<Assets<SizeAsset>>,
    camera_query: Query<&Camera>,
) {
    let camera = camera_query.single();
    let random_angle = rand::random::<f32>() * std::f32::consts::PI * 1.99 + 0.1;
    let random_speed = rand::random::<f32>() * 100.0 + 50.0;
    let random_velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let random_angular_velocity = rand::random::<f32>() * 2.9 + 0.1;
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position: Vec2 = 2.0
        * half_screen_size
        * Vec2::new(rand::random::<f32>().round(), rand::random::<f32>().round())
        - half_screen_size;

    let size = size_assets
        .get(&enemy_assets.enemy_size)
        .expect("Cannot find enemy size asset");

    commands.spawn(AsteroidEnemyBundle {
        sprite: SpriteBundle {
            texture: enemy_assets.enemy_texture.clone(),
            sprite: Sprite {
                custom_size: Some(size.sprite_size),
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
            size.collider_size.x / 2.0,
        ))),
        layers: CollisionLayers::new(layers::ENEMY_MASK, layers::PLAYER_MASK),
        damager: CollisionDamager::new(100),
        ..Default::default()
    });
}