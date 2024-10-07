use bevy::{
    color::palettes::css::{GREEN, RED},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use super::{aabb_from, spawn_player_system, AsteroidPlayer, BoxCollider, Movement};

pub struct AsteroidDebugPlugin;

impl Plugin for AsteroidDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default()).add_systems(
            Update,
            (
                switch_debug_system.run_if(input_just_pressed(KeyCode::KeyD)),
                spawn_player_system
                    .run_if(debug_is_active)
                    .run_if(not(any_with_component::<AsteroidPlayer>))
                    .run_if(input_just_pressed(KeyCode::KeyR)),
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

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Movement, &BoxCollider)>) {
    for (movement, collider) in &query {
        let aabb = aabb_from(movement, collider);
        gizmos.line_2d(aabb.min, aabb.max, RED);
        gizmos.rect_2d(movement.position, 0.0, collider.size, GREEN);
    }
}
