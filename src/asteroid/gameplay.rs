use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use super::{
    enemy::AsteroidEnemy, physics::CollisionEvent, player::AsteroidPlayer,
    projectile::AsteroidProjectile,
};

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_systems(
            Update,
            (gameplay_system.run_if(any_with_component::<AsteroidPlayer>),)
                .in_set(AsteroidGameplaySystem::UpdateGameplay),
        );
    }
}

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
}

pub fn gameplay_setup(
    mut commands: Commands,
    assets: Res<AsteroidGameplayAssets>,
    camera_query: Query<&Camera>,
) {
    commands.spawn(SpriteBundle {
        texture: assets.background_texture.clone(),
        sprite: Sprite {
            custom_size: camera_query.single().logical_viewport_size(),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..Default::default()
    });
}

pub fn gameplay_cleanup(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// TODO Make more events like enemy destroyed etc
// TODO Seperate different collision types somehow
fn gameplay_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<AsteroidPlayer>>,
    ennemies_query: Query<Entity, With<AsteroidEnemy>>,
    projectile_query: Query<Entity, With<AsteroidProjectile>>,
    mut score: ResMut<Score>,
) {
    for collision in collision_event.read() {
        if let Ok(player) = player_query.get(collision.first) {
            commands.entity(player).despawn();
        }

        if let Ok(projectile) = projectile_query.get(collision.first) {
            commands.entity(projectile).despawn();
            score.score += 10;
        }

        if let Ok(enemy) = ennemies_query.get(collision.second) {
            commands.entity(enemy).despawn();
        }
    }
}
