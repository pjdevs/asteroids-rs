use bevy::{
    color::palettes::css::{GREEN, RED},
    input::common_conditions::input_just_pressed,
    math::bounding::BoundingVolume,
    prelude::*,
};

use super::{
    physics::{aabb_from, BoxCollider, Movement},
    player::{spawn_player_system, AsteroidPlayer},
};

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
                debug_toggle_invincible_system
                    .run_if(any_with_component::<AsteroidPlayer>)
                    .run_if(input_just_pressed(KeyCode::KeyI)),
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
    let mut collider = query.single_mut();
    collider.enabled = !collider.enabled;
}

fn degug_gizmos_system(mut gizmos: Gizmos, query: Query<(&Movement, &BoxCollider)>) {
    for (movement, collider) in &query {
        let aabb = aabb_from(movement, collider);
        gizmos.line_2d(aabb.min, aabb.max, RED);
        gizmos.rect_2d(aabb.center(), 0.0, aabb.half_size() * 2.0, GREEN);
    }
}
