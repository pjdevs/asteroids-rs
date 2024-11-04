use super::prelude::Dead;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use bevy::prelude::*;

pub struct AsteroidBorderPlugin;

impl Plugin for AsteroidBorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                border_tunnel_system.run_if(any_with_component::<TunnelBorder>),
                border_despawn_system.run_if(any_with_component::<KillBorder>),
            )
                .run_if(in_state(AsteroidGameState::Game)),
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
        let offset = movement.position.abs() - half_screen_size;
        let position_sign = movement.position.signum();

        if offset.x > 0.0 {
            movement.position.x = -movement.position.x + 2.0 * position_sign.x * offset.x;
        }

        if offset.y > 0.0 {
            movement.position.y = -movement.position.y + 2.0 * position_sign.y * offset.y;
        }
    });
}

fn border_despawn_system(
    parallel_commands: ParallelCommands,
    query: Query<(Entity, &Movement), With<KillBorder>>,
    camera_query: Query<&Camera>,
) {
    let half_screen_size = get_screen_half_size(camera_query.single());

    query.iter().for_each(|(entity, movement)| {
        if movement.position.x.abs() > half_screen_size.x
            || movement.position.y.abs() > half_screen_size.y
        {
            parallel_commands.command_scope(|mut commands| {
                commands.entity(entity).insert(Dead);
            })
        }
    });
}

fn get_screen_half_size(camera: &Camera) -> Vec2 {
    let screen_size = camera.physical_target_size().unwrap();
    // Add a little extra offset to fake a larger screen that the camero do not see
    // so transition from one border to another are smooth
    Vec2::new(screen_size.x as f32 / 2.0 + 20.0, screen_size.y as f32 / 2.0 + 20.0)
}
