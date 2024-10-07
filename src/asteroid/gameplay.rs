use bevy::{math::bounding::IntersectsVolume, prelude::*};

use super::{aabb_from, AsteroidEnnemy, AsteroidPlayer, BoxCollider, Movement};

const COLLISION_SEARCH_LIMIT_SQUARED: f32 = 128.0 * 128.0;

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Startup, startup_system)
            .add_systems(
                Update,
                (
                    player_ennemy_collision_system.run_if(any_with_component::<AsteroidPlayer>),
                    player_ennemy_destruction_system,
                ),
            );
    }
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Event)]
pub struct CollisionEvent {
    first_entity: Entity,
    seconds_entity: Entity,
}

pub fn player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidPlayer>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<AsteroidEnnemy>>,
) {
    let (player, player_movement, player_collider) = player_query.single();
    let player_aabb = aabb_from(player_movement, player_collider);

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

fn player_ennemy_destruction_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for event in collision_event.read() {
        if let Some(mut first) = commands.get_entity(event.first_entity) {
            first.despawn();
        }

        if let Some(mut second) = commands.get_entity(event.seconds_entity) {
            second.despawn();
        }
    }
}
