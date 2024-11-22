use super::prelude::*;
use crate::asset;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Game),
            (
                remove_resource::<EnemyAssets>,
                despawn_entities_with::<Enemy>,
            ),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::GameLoading).load_collection::<EnemyAssets>(),
        )
        .add_spawner::<EnemySpawner>(
            GameState::GameLoading,
            GameState::Game,
            IntoSystem::into_system(spawn_enemy_system),
            EnemySystem::UpdateSpawnEnemies,
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct EnemyAssets {
    #[asset(key = "enemy.texture")]
    pub enemy_texture: Handle<Image>,

    #[asset(key = "enemy.layout")]
    pub enemy_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "enemy.size.ron")]
    pub enemy_size: Handle<SizeAsset>,

    #[asset(path = "enemy.spawner.ron")]
    pub enemy_spawner: Handle<SpawnerAsset>,
}

// Spawner
#[derive(Component, Default, Reflect)]
pub struct EnemySpawner;

impl FromWorld for Spawner<EnemySpawner> {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<Res<EnemyAssets>>::new(world);
        let enemy_assets = system_state.get(world);

        Spawner::from_asset(enemy_assets.enemy_spawner.clone_weak())
    }
}

// Components

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    movement: Movement,
    collider: Collider,
    layers: CollisionLayers,
    border: TunnelBorder,
    health: Health,
    damager: CollisionDamager,
    despawn_on_dead: DespawnOnDead,
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum EnemySystem {
    UpdateSpawnEnemies,
}

fn spawn_enemy_system(
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
    size_assets: Res<Assets<SizeAsset>>,
) -> Entity {
    let size = asset!(size_assets, &enemy_assets.enemy_size);

    let enemy = EnemyBundle {
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
