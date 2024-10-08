use bevy::{math::bounding::IntersectsVolume, prelude::*};

use super::{
    ennemy::AsteroidEnnemy,
    physics::{aabb_from, BoxCollider, Movement},
    player::AsteroidPlayer,
};

const COLLISION_SEARCH_LIMIT_SQUARED: f32 = 128.0 * 128.0;

pub struct AsteroidGameplayPlugin;

impl Plugin for AsteroidGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Startup, startup_system)
            .add_systems(
                Update,
                (
                    gameplayer_player_ennemy_collision_system
                        .run_if(any_with_component::<AsteroidPlayer>),
                    gameplay_player_ennemy_destruction_system,
                    gameplay_border_system,
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

pub fn gameplayer_player_ennemy_collision_system(
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

fn gameplay_player_ennemy_destruction_system(
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

pub fn gameplay_border_system(mut query: Query<&mut Movement>, camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);

    query.iter_mut().for_each(|mut movement| {
        if movement.position.x.abs() > half_screen_size.x + 32.0 {
            movement.position.x *= -1.0;
        }

        if movement.position.y.abs() > half_screen_size.y + 32.0 {
            movement.position.y *= -1.0;
        }
    });
}
