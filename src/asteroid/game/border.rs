use super::prelude::Dead;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use bevy::prelude::*;

pub struct AsteroidBorderPlugin;

impl Plugin for AsteroidBorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (border_tunnel_system, border_despawn_system).run_if(in_state(AsteroidGameState::Game)),
        );
    }
}

#[derive(Component, Default)]
pub struct TunnelBorder;

#[derive(Component, Default)]
pub struct KillBorder;

fn border_tunnel_system(
    mut query: Query<&mut Movement, (With<TunnelBorder>, Without<Dead>)>,
    camera_query: Query<&Camera>,
) {
    let half_screen_size = get_screen_half_size(camera_query.single());

    query.par_iter_mut().for_each(|mut movement| {
        if movement.position.x.abs() > half_screen_size.x + 32.0 {
            movement.position.x *= -1.0;
        }

        if movement.position.y.abs() > half_screen_size.y + 32.0 {
            movement.position.y *= -1.0;
        }
    });
}

// TODO Remove hardcoded safety offset

fn border_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &Movement), With<KillBorder>>,
    camera_query: Query<&Camera>,
) {
    let half_screen_size = get_screen_half_size(camera_query.single());

    query.iter().for_each(|(entity, movement)| {
        if movement.position.x.abs() > half_screen_size.x + 32.0
            || movement.position.y.abs() > half_screen_size.y + 32.0
        {
            commands.entity(entity).insert(Dead);
        }
    });
}

fn get_screen_half_size(camera: &Camera) -> Vec2 {
    let screen_size = camera.physical_target_size().unwrap();
    Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0)
}
