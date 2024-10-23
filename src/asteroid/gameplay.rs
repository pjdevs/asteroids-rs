use bevy::{math::bounding::IntersectsVolume, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

use super::{
    enemy::AsteroidEnemy,
    physics::{aabb_from, BoxCollider, Movement},
    player::AsteroidPlayer,
    projectile::AsteroidProjectile,
};

const COLLISION_SEARCH_LIMIT_SQUARED: f32 = 128.0 * 128.0;

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<CollisionEvent>()
            .add_systems(
                Update,
                (
                    gameplay_player_ennemy_collision_system
                        .run_if(any_with_component::<AsteroidPlayer>),
                    gameplay_projectile_ennemy_collision_system
                        .run_if(any_with_component::<AsteroidProjectile>),
                    gameplay_collision_destruction_system,
                )
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

#[derive(Event)]
pub struct CollisionEvent {
    first_entity: Entity,
    seconds_entity: Entity,
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

fn gameplay_player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidPlayer>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidEnemy>>,
) {
    for (player, player_movement, player_collider) in &player_query {
        if !player_collider.enabled {
            continue;
        }

        let player_aabb = aabb_from(player_movement, player_collider);

        // TODO Implement this with quadtrees directly in physcis plugin
        // TODO Investigate parallel iteration to trigger event
        ennemies_query
            .iter()
            .filter(|(_, ennemy_movement, _)| {
                player_movement
                    .position
                    .distance_squared(ennemy_movement.position)
                    < COLLISION_SEARCH_LIMIT_SQUARED
            })
            .for_each(|(ennemy, ennemy_movement, ennemy_collider)| {
                let ennemy_aabb = aabb_from(ennemy_movement, ennemy_collider);

                if player_aabb.intersects(&ennemy_aabb) {
                    collision_event.send(CollisionEvent {
                        first_entity: player,
                        seconds_entity: ennemy,
                    });
                }
            });
    }
}

// TODO Maybe another component for player projectile to be able to use them for ennemies

fn gameplay_projectile_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    projectile_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidProjectile>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidEnemy>>,
) {
    // TODO Implement this with quadtrees directly in physcis plugin
    // TODO Investigate parallel iteration to trigger event
    for (ennemy, ennemy_movement, ennemy_collider) in &ennemies_query {
        for (projectile, projectile_movement, projectile_collider) in &projectile_query {
            if projectile_movement
                .position
                .distance_squared(ennemy_movement.position)
                < COLLISION_SEARCH_LIMIT_SQUARED
            {
                let projectile_aabb = aabb_from(projectile_movement, projectile_collider);
                let ennemy_aabb = aabb_from(ennemy_movement, ennemy_collider);

                if projectile_aabb.intersects(&ennemy_aabb) {
                    collision_event.send(CollisionEvent {
                        first_entity: projectile,
                        seconds_entity: ennemy,
                    });
                }
            }
        }
    }
}

fn gameplay_collision_destruction_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    for (event, _) in collision_event.par_read() {
        if let Some(mut first) = commands.get_entity(event.first_entity) {
            first.despawn();
        }

        if let Some(mut second) = commands.get_entity(event.seconds_entity) {
            second.despawn();
            score.score += 10;
        }
    }
}
