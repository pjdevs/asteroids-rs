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
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector::{ui_for_assets, ui_for_resource};
use bevy_inspector_egui::egui;

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .register_type::<AsteroidSpawner<AsteroidEnemySpawner>>()
            .insert_resource(AsteroidDebugConfig::default())
            .add_systems(
                Update,
                (
                    toggle_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
                    (
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

fn debug_toggle_invincible_system(
    mut query: Query<&mut Collider, With<AsteroidPlayer>>,
) {
    for mut collider in &mut query {
        collider.enabled = !collider.enabled;
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

fn debug_custom_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Asteroid Debug Menu").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Config");
            ui_for_resource::<AsteroidDebugConfig>(world, ui);

            ui.heading("Cheats");
            ui_for_cheats(world, ui);

            ui.collapsing("Enemy Spawner", |ui| {
                ui_for_resource::<AsteroidSpawner<AsteroidEnemySpawner>>(world, ui);
            });

            ui.collapsing("Spawner Assets", |ui| {
                ui_for_assets::<SpawnerAsset>(world, ui);
            });
        });
    });
}

fn ui_for_cheats(world: &mut World, ui: &mut egui::Ui) {
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

    if ui.button("Toggle Invincibility").clicked() {
        world.run_system_once(debug_toggle_invincible_system);
    }

    ui.heading("Enemies");

    if ui.button("Kill All Enemies").clicked() {
        world.run_system_once(kill_all_enemies_system);
    }
}
