use bevy::prelude::*;

use crate::asteroid::input::InputAction;

use super::{
    border::TunnelBorder,
    controller::Speed,
    input::InputController,
    physics::{BoxCollider, Movement},
    projectile::{AsteroidProjectileAssets, AsteroidProjectileBundle},
};

const PLAYER_SIZE: f32 = 48.0;

pub struct AsteroidPlayerPlugin;

impl Plugin for AsteroidPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system)
            .add_systems(Update, player_shoot_system);
    }
}

#[derive(Component, Default)]
pub struct AsteroidPlayer;

#[derive(Bundle, Default)]
pub struct AsteroidPlayerBundle {
    player: AsteroidPlayer,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
    border: TunnelBorder,
    speed: Speed,
    controller: InputController,
}

impl AsteroidPlayerBundle {
    pub fn from(asset_server: &AssetServer) -> Self {
        Self {
            player: AsteroidPlayer,
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/ship.png"),
                ..Default::default()
            },
            movement: Movement {
                friction: 0.03,
                ..Default::default()
            },
            collider: BoxCollider {
                size: Vec2::splat(PLAYER_SIZE),
                ..Default::default()
            },
            speed: Speed {
                movement_speed: 750.0,
                rotation_speed: 4.0,
            },
            ..Default::default()
        }
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AsteroidPlayerBundle::from(&asset_server));
}

fn player_shoot_system(
    mut commands: Commands,
    projectile_assets: Res<AsteroidProjectileAssets>,
    player_query: Query<(&InputController, &Movement), With<AsteroidPlayer>>,
) {
    const PROJECTILE_SPEED: f32 = 600.0;

    for (controller, player_movement) in &player_query {
        if controller.input_action(InputAction::Shoot) {
            commands.spawn(AsteroidProjectileBundle {
                sprite: SpriteBundle {
                    texture: projectile_assets.texture.clone(),
                    ..Default::default()
                },
                movement: Movement {
                    position: player_movement.position,
                    velocity: player_movement.get_direction() * PROJECTILE_SPEED,
                    rotation: player_movement.rotation,
                    ..Default::default()
                },
                collider: BoxCollider {
                    size: projectile_assets.projectile_size,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
