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

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameConfig {
            is_debug_mode: false,
        })
        .add_event::<CollisionEvent>()
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, physics_fixed_system)
        .add_systems(
            Update,
            (
                (
                    player_movement_system.run_if(any_with_component::<Player>),
                    spawn_ennemies_system.run_if(on_timer(Duration::from_secs(5))),
                    ennemies_border_system,
                    player_ennemy_collision_system.run_if(any_with_component::<Player>),
                    player_ennemy_destruction_system,
                    switch_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
                    debug(degug_gizmos_system),
                ),
                physics_transform_extrapolate_system,
            )
                .chain(),
        );
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ship.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        },
        Movement::default(),
        Player { speed: 275.0 },
    ));

    commands.insert_resource(EnnemyAssets {
        texture: asset_server.load("sprites/ball.png"),
    });
}

#[derive(Event)]
pub struct CollisionEvent {
    first_entity: Entity,
    seconds_entity: Entity,
}

pub fn player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
    ennemies_query: Query<(Entity, &Transform, &Sprite), With<Ennemy>>,
) {
    let (player, player_transform, player_sprite) = player_query.single();
    let player_aabb = aabb_from_transform_sprite(player_transform, player_sprite);

    ennemies_query
        .iter()
        .filter(|(_, ennemy_transform, _)| {
            player_transform
                .translation
                .distance_squared(ennemy_transform.translation)
                < 128.0 * 128.0
        })
        .for_each(|(ennemy, ennemy_transform, ennemy_sprite)| {
            let ennemy_aabb = aabb_from_transform_sprite(ennemy_transform, ennemy_sprite);

            if player_aabb.intersects(&ennemy_aabb) {
                collision_event.send(CollisionEvent {
                    first_entity: player,
                    seconds_entity: ennemy,
                });
            }
        });
}

fn aabb_from_transform_sprite(transform: &Transform, sprite: &Sprite) -> Aabb2d {
    Aabb2d::new(
        transform.translation.truncate(),
        sprite.custom_size.unwrap_or(Vec2::ZERO) / 2.0,
    )
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

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Transform, &Sprite)>) {
    for (transform, sprite) in &query {
        let aabb = aabb_from_transform_sprite(transform, sprite);
        gizmos.line_2d(aabb.min, aabb.max, RED);
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.0,
            sprite.custom_size.unwrap_or(Vec2::ZERO),
            GREEN,
        );
    }
}
