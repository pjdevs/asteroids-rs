use bevy::{color::palettes::css::GREEN, input::common_conditions::input_just_pressed, prelude::*};

use super::{
    physics::{BoxCollider, Movement},
    player::{
        player_exists, spawn_first_player_system, spawn_second_player_system, AsteroidPlayer,
    },
};

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default()).add_systems(
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
            ),
        );
    }
}

#[derive(Resource, Default)]
struct GameConfig {
    is_debug_mode: bool,
}

fn debug_is_active(config: Res<GameConfig>) -> bool {
    config.is_debug_mode
}

fn switch_debug_system(mut config: ResMut<GameConfig>) {
    config.is_debug_mode = !config.is_debug_mode;
}

fn debug_toggle_invincible_system(mut query: Query<&mut BoxCollider, With<AsteroidPlayer>>) {
    for mut collider in &mut query {
        collider.enabled = !collider.enabled;
    }
}

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Movement, &BoxCollider)>) {
    for (movement, collider) in &query {
        gizmos.rect_2d(movement.position, movement.rotation, collider.size, GREEN);
    }
}
