use bevy::{math::bounding::IntersectsVolume, prelude::*};

use super::{
    ennemy::AsteroidEnnemy,
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
            .add_systems(Startup, startup_system)
            .add_systems(
                Update,
                (
                    gameplay_player_ennemy_collision_system
                        .run_if(any_with_component::<AsteroidPlayer>),
                    gameplay_projectile_ennemy_collision_system
                        .run_if(any_with_component::<AsteroidProjectile>),
                    gameplay_collision_destruction_system,
                ),
            );
    }
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

pub fn gameplay_player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidPlayer>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidEnnemy>>,
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

pub fn gameplay_projectile_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    projectile_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidProjectile>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidEnnemy>>,
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
