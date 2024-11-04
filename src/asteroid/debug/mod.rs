use super::core::prelude::*;
use super::game::player::{spawn_first_player_system, spawn_second_player_system, AsteroidPlayer};
use super::game::prelude::*;
use super::physics::prelude::*;
use bevy::color::palettes::css::{GREEN, WHITE};
use bevy::ecs::system::RunSystemOnce;
use bevy::input::common_conditions::input_just_pressed;
use bevy::math::bounding::BoundingVolume;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::{egui, quick::*};

// TODO Merge default quick inspector into own UI

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            .register_type::<AsteroidSpawner<AsteroidEnemySpawner>>()
            // Inspectors
            // .add_plugins(WorldInspectorPlugin::default().run_if(debug_is_active))
            .add_plugins(
                ResourceInspectorPlugin::<AsteroidSpawner<AsteroidEnemySpawner>>::default()
                    .run_if(debug_is_active),
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
                    (
                        debug_invincible_system.run_if(any_with_component::<AsteroidPlayer>),
                        degug_gizmos_system,
                        debug_custom_ui,
                    )
                        .run_if(debug_is_active),
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

// Systems

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

fn degug_gizmos_system(
    mut gizmos: Gizmos,
    query: Query<(&Movement, &Collider)>,
    config: Res<AsteroidDebugConfig>,
) {
    if !config.show_gizmos {
        return;
    }

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

fn kill_all_enemies_system(mut commands: Commands, query: Query<Entity, With<AsteroidEnemy>>) {
    for enemy in &query {
        commands.entity(enemy).insert(Dead);
    }
}

// TODO Emit events and use commands to avoid use exclusive system ?
fn debug_custom_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Debug Commands").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Players");

            if ui.button("Spawn Player 1").clicked() {
                let player_exists = world.run_system_once(player_exists(1));
                if !player_exists {
                    world.run_system_once(spawn_first_player_system);
                }
            }

            if ui.button("Spawn Player 2").clicked() {
                let player_exists = world.run_system_once(player_exists(2));
                if !player_exists {
                    world.run_system_once(spawn_second_player_system);
                }
            }

            ui.heading("Enemies");

            if ui.button("Kill All Enemies").clicked() {
                world.run_system_once(kill_all_enemies_system);
            }
        });
    });
}
