use bevy::prelude::*;

use super::{BoxCollider, Movement};

const PLAYER_SIZE: f32 = 48.0;

pub struct AsteroidPlayerPlugin;

impl Plugin for AsteroidPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system).add_systems(
            Update,
            player_movement_system.run_if(any_with_component::<AsteroidPlayer>),
        );
    }
}

#[derive(Component)]
pub struct AsteroidPlayer {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Bundle)]
pub struct AsteroidPlayerBundle {
    player: AsteroidPlayer,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
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
        }
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AsteroidPlayerBundle::from(&asset_server));
}

pub fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&AsteroidPlayer, &mut Movement)>,
) {
    let (player, mut player_movement) = player_query.single_mut();
    let mut input_direction = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        input_direction.y += 1.0;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        input_direction.y -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        input_direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowRight) {
        input_direction.x += 1.0;
    }

    input_direction = input_direction.normalize_or_zero();

    // Rotation
    player_movement.angular_velocity = -input_direction.x * player.rotation_speed;

    // Translation
    let movement_direction =
        Quat::from_rotation_z(player_movement.rotation) * Vec3::new(0.0, input_direction.y, 0.0);
    player_movement.acceleration = movement_direction.truncate() * player.movement_speed;
}
