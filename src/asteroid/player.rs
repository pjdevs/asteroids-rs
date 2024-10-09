use bevy::prelude::*;

use super::{
    border::TunnelBorder,
    input::{when_action, InputAction, InputMap},
    physics::{BoxCollider, Movement},
    projectile::{AsteroidProjectileAssets, AsteroidProjectileBundle},
};

const PLAYER_SIZE: f32 = 48.0;

pub struct AsteroidPlayerPlugin;

// TODO Make this a SystemSet with a condition `any_with_component::<AsteroidPlayer>`

impl Plugin for AsteroidPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system).add_systems(
            Update,
            (
                player_movement_system.run_if(any_with_component::<AsteroidPlayer>),
                player_shoot_system
                    .run_if(any_with_component::<AsteroidPlayer>)
                    .run_if(when_action::<KeyCode>(InputAction::Shoot)),
            ),
        );
    }
}

#[derive(Component, Default)]
pub struct AsteroidPlayer {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Bundle, Default)]
pub struct AsteroidPlayerBundle {
    player: AsteroidPlayer,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
    border: TunnelBorder,
}

impl AsteroidPlayerBundle {
    pub fn from(asset_server: &AssetServer) -> Self {
        Self {
            player: AsteroidPlayer {
                movement_speed: 750.0,
                rotation_speed: 4.0,
            },
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/ship.png"),
                ..default()
            },
            movement: Movement {
                friction: 0.03,
                ..default()
            },
            collider: BoxCollider {
                size: Vec2::splat(PLAYER_SIZE),
            },
            ..default()
        }
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AsteroidPlayerBundle::from(&asset_server));
}

fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<InputMap<KeyCode>>,
    mut player_query: Query<(&AsteroidPlayer, &mut Movement)>,
) {
    let (player, mut player_movement) = player_query.single_mut();
    let mut input_direction = Vec2::ZERO;

    if map.input_action(InputAction::Forward, &keys) {
        input_direction.y += 1.0;
    }

    if map.input_action(InputAction::Backward, &keys) {
        input_direction.y -= 1.0;
    }

    if map.input_action(InputAction::TurnLeft, &keys) {
        input_direction.x -= 1.0;
    }

    if map.input_action(InputAction::TurnRight, &keys) {
        input_direction.x += 1.0;
    }

    // Rotation
    player_movement.angular_velocity = -input_direction.x * player.rotation_speed;

    // Translation
    player_movement.acceleration =
        player_movement.get_direction() * player.movement_speed * input_direction.y;
}

fn player_shoot_system(
    mut commands: Commands,
    projectile_assets: Res<AsteroidProjectileAssets>,
    player_query: Query<&Movement, With<AsteroidPlayer>>,
) {
    let player_movement = player_query.single();
    const PROJECTILE_SPEED: f32 = 600.0;

    commands.spawn(AsteroidProjectileBundle {
        sprite: SpriteBundle {
            texture: projectile_assets.texture.clone(),
            ..default()
        },
        movement: Movement {
            position: player_movement.position,
            velocity: player_movement.get_direction() * PROJECTILE_SPEED,
            rotation: player_movement.rotation,
            ..default()
        },
        collider: BoxCollider {
            size: projectile_assets.projectile_size,
        },
        ..default()
    });
}
