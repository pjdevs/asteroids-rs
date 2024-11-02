use super::core::prelude::*;
use super::game::enemy::AsteroidEnemySpawner;
use super::game::player::{spawn_first_player_system, spawn_second_player_system, AsteroidPlayer};
use super::physics::prelude::*;
use bevy::color::palettes::css::{GREEN, WHITE};
use bevy::input::common_conditions::input_just_pressed;
use bevy::math::bounding::BoundingVolume;
use bevy::prelude::*;
use bevy_inspector_egui::quick::*;

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            .register_type::<AsteroidEnemySpawner>()
            // Inspectors
            // .add_plugins(WorldInspectorPlugin::default().run_if(debug_is_active))
            .add_plugins(
                ResourceInspectorPlugin::<AsteroidEnemySpawner>::default().run_if(debug_is_active),
            )
            .add_plugins(
                ResourceInspectorPlugin::<AsteroidDebugConfig>::default().run_if(debug_is_active),
            )
            .add_plugins(AssetInspectorPlugin::<SpawnerAsset>::default().run_if(debug_is_active))
            // Debug
            .insert_resource(AsteroidDebugConfig::default())
            .add_systems(
                Update,
                (
                    toggle_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
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
                    debug_invincible_system.run_if(
                        debug_is_active
                            .and_then(any_with_component::<AsteroidPlayer>),
                    ),
                    degug_gizmos_system.run_if(debug_show_gizmos),
                )
                    .run_if(in_state(AsteroidGameState::Game)),
            );
    }
}

// Resources

#[derive(Resource, Default, Reflect)]
struct AsteroidDebugConfig {
    is_debug_mode: bool,
    show_gizmos: bool,
    is_invincible: bool,
}

// Conditions

pub fn player_exists(player_id: u64) -> impl Fn(Query<&AsteroidPlayer>) -> bool {
    move |query: Query<&AsteroidPlayer>| query.iter().any(|p| p.player_id == player_id)
}

fn debug_is_active(config: Res<AsteroidDebugConfig>) -> bool {
    config.is_debug_mode
}

fn debug_show_gizmos(config: Res<AsteroidDebugConfig>) -> bool {
    config.is_debug_mode && config.show_gizmos
}

// Systems

fn toggle_debug_system(mut config: ResMut<AsteroidDebugConfig>) {
    config.is_debug_mode = !config.is_debug_mode;
}

fn toggle_debug_system(mut config: ResMut<AsteroidDebugConfig>) {
    config.is_debug_mode = !config.is_debug_mode;
}

fn debug_invincible_system(
    mut query: Query<&mut Collider, With<AsteroidPlayer>>,
    config: Res<AsteroidDebugConfig>,
) {
    if config.is_changed() {
        for mut collider in &mut query {
            collider.enabled = !config.is_invincible;
        }
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
