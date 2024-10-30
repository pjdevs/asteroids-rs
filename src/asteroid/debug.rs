use bevy::{
    color::palettes::css::{GREEN, WHITE},
    input::common_conditions::input_just_pressed,
    math::bounding::BoundingVolume,
    prelude::*,
};

use super::{
    physics::{
        collision::{Collider, Shape},
        movement::Movement,
    },
    player::{
        player_exists, spawn_first_player_system, spawn_second_player_system, AsteroidPlayer,
    },
    states::AsteroidGameState,
};

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidDebugConfig::default())
            .add_systems(
                Update,
                (
                    switch_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
                    spawn_first_player_system.run_if(
                        debug_is_active
                            .and_then(not(player_exists(1)))
                            .and_then(input_just_pressed(KeyCode::Digit1)),
                    ),
                    spawn_second_player_system.run_if(
                        debug_is_active
                            .and_then(not(player_exists(2)))
                            .and_then(input_just_pressed(KeyCode::Digit2)),
                    ),
                    debug_toggle_invincible_system.run_if(
                        debug_is_active
                            .and_then(any_with_component::<AsteroidPlayer>)
                            .and_then(input_just_pressed(KeyCode::KeyI)),
                    ),
                    degug_gizmos_system.run_if(debug_is_active),
                )
                    .run_if(in_state(AsteroidGameState::InGame)),
            );
    }
}

#[derive(Resource, Default)]
struct AsteroidDebugConfig {
    is_debug_mode: bool,
}

fn debug_is_active(config: Res<AsteroidDebugConfig>) -> bool {
    config.is_debug_mode
}

fn switch_debug_system(mut config: ResMut<AsteroidDebugConfig>) {
    config.is_debug_mode = !config.is_debug_mode;
}

fn debug_toggle_invincible_system(mut query: Query<&mut Collider, With<AsteroidPlayer>>) {
    for mut collider in &mut query {
        collider.enabled = !collider.enabled;
    }
}

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Movement, &Collider)>) {
    for (movement, collider) in &query {
        let color = if collider.enabled { GREEN } else { WHITE };

        match collider.shape.transformed_by(movement) {
            Shape::Aabb(aabb) => {
                gizmos.rect_2d(aabb.center(), 0.0, aabb.half_size() * 2.0, color);
            }
            Shape::Obb(obb) => {
                gizmos.rect_2d(obb.center, obb.rotation, obb.half_size * 2.0, color);
            }
            Shape::Circle(circle) => {
                gizmos.circle_2d(circle.center, circle.radius(), color);
            }
        };
    }
}
