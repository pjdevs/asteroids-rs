use bevy::app::Plugin;
use bevy::color::palettes::css::{GREEN, RED};
use bevy::ecs::schedule::NodeConfigs;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use ennemy::*;
use player::*;

mod ennemy;
mod player;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(GameConfig {
                is_debug_mode: false,
            })
            .add_event::<CollisionEvent>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    update_timers_system,
                    player_movement_system,
                    spawn_ennemies_system.run_if(should_spawn_ennemies),
                    ennemies_movement_system,
                    ennemies_despawn_system,
                    player_ennemy_collision_system,
                    player_ennemy_destruction_system,
                    debug_input_system,
                    debug(degug_gizmos_system),
                ),
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
        Player { speed: 200.0 },
    ));

    commands.insert_resource(EnnemyAssets {
        texture: asset_server.load("sprites/ball.png"),
    });
}

#[derive(Event)]
pub struct CollisionEvent {
    ennemy: Entity,
}

pub fn player_ennemy_collision_system(
    mut collision_event: EventWriter<CollisionEvent>,
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    ennemies_query: Query<(Entity, &Transform, &Sprite), With<Ennemy>>,
) {
    let Ok((player_transform, player_sprite)) = player_query.get_single() else {
        return;
    };

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
                collision_event.send(CollisionEvent { ennemy });
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
    player_query: Query<Entity, With<Player>>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    if !collision_event.is_empty() {
        commands.entity(player).despawn();
    }

    for event in collision_event.read() {
        commands.entity(event.ennemy).despawn();
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

fn debug_input_system(mut config: ResMut<GameConfig>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyD) {
        config.is_debug_mode = !config.is_debug_mode;
    }
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
