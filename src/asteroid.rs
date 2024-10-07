use std::time::Duration;

use bevy::app::Plugin;
use bevy::color::palettes::css::{GREEN, RED};
use bevy::ecs::schedule::NodeConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use bevy::time::common_conditions::on_timer;
use ennemy::*;
use physics::*;
use player::*;

mod ennemy;
mod physics;
mod player;

const COLLISION_SEARCH_LIMIT_SQUARED: f32 = 128.0 * 128.0;
const ENNEMY_SPAWN_DELAY: u64 = 5;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameConfig {
            is_debug_mode: false,
        })
        .add_event::<CollisionEvent>()
        .add_systems(Startup, (startup_system, spawn_player_system))
        .add_systems(FixedUpdate, physics_fixed_system)
        .add_systems(
            Update,
            (
                (
                    player_movement_system.run_if(any_with_component::<Player>),
                    spawn_ennemies_system.run_if(on_timer(Duration::from_secs(ENNEMY_SPAWN_DELAY))),
                    player_ennemy_collision_system.run_if(any_with_component::<Player>),
                    player_ennemy_destruction_system,
                    border_system,
                    switch_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
                    debug(
                        spawn_player_system
                            .run_if(not(any_with_component::<Player>))
                            .run_if(input_just_pressed(KeyCode::KeyR)),
                    ),
                    debug(degug_gizmos_system),
                ),
                physics_transform_extrapolate_system,
            )
                .chain(),
        );
    }
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(EnnemyAssets::default(&asset_server));
}

fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::from(&asset_server));
}

#[derive(Event)]
pub struct CollisionEvent {
    first_entity: Entity,
    seconds_entity: Entity,
}

pub fn player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(Entity, &Movement, &BoxCollider), With<Player>>,
    ennemies_query: Query<(Entity, &Movement, &BoxCollider), With<Ennemy>>,
) {
    let (player, player_movement, player_collider) = player_query.single();
    let player_aabb = aabb_from(player_movement, player_collider);

    ennemies_query
        .iter()
        .filter(|(_, ennemy_transform, _)| {
            player_movement
                .position
                .distance_squared(ennemy_transform.position)
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

fn aabb_from(movement: &Movement, collider: &BoxCollider) -> Aabb2d {
    Aabb2d::new(movement.position, collider.size / 2.0)
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

#[derive(Resource)]
struct GameConfig {
    is_debug_mode: bool,
}

fn debug<M>(
    system: impl IntoSystemConfigs<M>,
) -> NodeConfigs<Box<(dyn bevy::prelude::System<In = (), Out = ()> + 'static)>> {
    system.run_if(|config: Res<GameConfig>| config.is_debug_mode)
}

fn switch_debug_system(mut config: ResMut<GameConfig>) {
    config.is_debug_mode = !config.is_debug_mode;
}

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Movement, &BoxCollider)>) {
    for (movement, collider) in &query {
        let aabb = aabb_from(movement, collider);
        gizmos.line_2d(aabb.min, aabb.max, RED);
        gizmos.rect_2d(movement.position, 0.0, collider.size, GREEN);
    }
}
