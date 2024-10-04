use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub position: Vec2,
    pub velocity: Vec2,
    pub friction: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            friction: 1.0,
        }
    }
}

pub fn physics_fixed_system(time: Res<Time>, mut query: Query<&mut Movement>) {
    for mut movement in &mut query {
        let movement = &mut *movement;
        movement.velocity *= movement.friction;
        movement.position += movement.velocity * time.delta_seconds();
    }
}

pub fn physics_transform_extrapolate_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Movement)>,
) {
    for (mut transform, movement) in &mut query {
        let future_position =
            movement.position + movement.velocity * movement.friction * fixed_time.delta_seconds();
        transform.translation = movement
            .position
            .lerp(future_position, fixed_time.overstep_fraction())
            .extend(0.0);
    }
}
