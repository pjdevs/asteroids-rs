use super::prelude::*;
use crate::asteroid::animation::prelude::*;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::{asset, get};
use bevy::ecs::system::SystemState;
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trauma_shake::Shake;

pub struct AsteroidEnemyPlugin;

impl Plugin for AsteroidEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (explode_enemy_system, kill_exploded_enemy_system)
                .after(AsteroidDamageSystem::FixedUpdateDamageSystem)
                .run_if(in_state(AsteroidGameState::Game)),
        )
        .add_systems(
            OnExit(AsteroidGameState::Game),
            (
                remove_resource::<AsteroidEnemyAssets>,
                despawn_entities_with::<AsteroidEnemy>,
            ),
        )
        .configure_loading_state(
            LoadingStateConfig::new(AsteroidGameState::GameLoading)
                .load_collection::<AsteroidEnemyAssets>(),
        )
        .add_spawner::<AsteroidEnemySpawner>(
            AsteroidGameState::GameLoading,
            AsteroidGameState::Game,
            IntoSystem::into_system(spawn_enemy_system),
            AsteroidEnemySystem::UpdateSpawnEnemies,
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidEnemyAssets {
    #[asset(key = "enemy.texture")]
    pub enemy_texture: Handle<Image>,

    #[asset(key = "enemy.layout")]
    pub enemy_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "enemy.size.ron")]
    pub enemy_size: Handle<SizeAsset>,

    #[asset(path = "enemy.spawner.ron")]
    pub enemy_spawner: Handle<SpawnerAsset>,

    #[asset(path = "enemy_explosion.anim.ron")]
    pub enemy_explosion_animation: Handle<AnimationAsset>,
}

// Spawner
#[derive(Component, Default, Reflect)]
pub struct AsteroidEnemySpawner;

impl FromWorld for AsteroidSpawner<AsteroidEnemySpawner> {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<Res<AsteroidEnemyAssets>>::new(world);
        let enemy_assets = system_state.get(world);

        AsteroidSpawner::from_asset(enemy_assets.enemy_spawner.clone_weak())
    }
}

// Components

#[derive(Component, Default)]
pub struct AsteroidEnemy;

#[derive(Bundle, Default)]
pub struct AsteroidEnemyBundle {
    enemy: AsteroidEnemy,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
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

fn spawn_enemy_system(
    mut commands: Commands,
    enemy_assets: Res<AsteroidEnemyAssets>,
    size_assets: Res<Assets<SizeAsset>>,
) -> Entity {
    let size = asset!(size_assets, &enemy_assets.enemy_size);

    let enemy = AsteroidEnemyBundle {
        sprite: SpriteBundle {
            texture: enemy_assets.enemy_texture.clone(),
            sprite: Sprite {
                custom_size: Some(size.sprite_size),
                ..Default::default()
            },
            ..Default::default()
        },
        atlas: TextureAtlas {
            layout: enemy_assets.enemy_layout.clone_weak(),
            index: 0,
        },
        collider: Collider::from_shape(Shape::Circle(BoundingCircle::new(
            Vec2::ZERO,
            size.collider_size.x / 2.0,
        ))),
        layers: CollisionLayers::new(layers::ENEMY_MASK, layers::PLAYER_MASK),
        damager: Damager::Constant(100).into(),
        ..Default::default()
    };

    commands
        .spawn((
            enemy,
            #[cfg(feature = "dev")]
            Name::new("Enemy"),
        ))
        .id()
}

// TODO Think about moviing this to like VFX plugin?
fn explode_enemy_system(
    mut commands: Commands,
    assets: Res<AsteroidEnemyAssets>,
    mut query: Query<
        (
            Entity,
            &mut Movement,
            &mut Collider,
            &mut Sprite,
            &AsteroidScaled,
        ),
        (With<AsteroidEnemy>, Added<Dead>),
    >,
    mut shake_query: Query<&mut Shake>,
) {
    let mut shake = shake_query.single_mut();

    for (entity, mut movement, mut collider, mut sprite, scaled) in &mut query {
        collider.enabled = false;
        movement.angular_velocity = 0.0;

        sprite.color = Color::srgb(5.0, 3.0, 0.0);

        commands
            .entity(entity)
            .insert(Animation::new(assets.enemy_explosion_animation.clone_weak()));

        shake.add_trauma(0.2 * scaled.scale);
    }
}

// TODO Handle specific animation
fn kill_exploded_enemy_system(
    mut commands: Commands,
    mut events: EventReader<AnimationCompleted>,
    query: Query<(), With<AsteroidEnemy>>,
) {
    for event in events.read() {
        get!(_enemy, query, event.animated_entity, continue);
        commands.entity(event.animated_entity).insert(DespawnOnDead);
    }
}
