use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub friction: f32,
    pub max_speed: f32,
}

impl Movement {
    pub fn get_direction(&self) -> Vec2 {
        (Quat::from_rotation_z(self.rotation) * Vec3::Y).truncate()
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            rotation: 0.0,
            angular_velocity: 0.0,
            friction: 0.0,
            max_speed: f32::MAX,
        }
    }
}

pub(super) fn physics_fixed_movement_system(time: Res<Time>, mut query: Query<&mut Movement>) {
    query.par_iter_mut().for_each(|mut movement| {
        let movement = &mut *movement;

        // Translation
        movement.velocity +=
            movement.acceleration * time.delta_secs() - movement.friction * movement.velocity;
        movement.velocity = movement.velocity.clamp_length_max(movement.max_speed);
        movement.position += movement.velocity * time.delta_secs();

        // Rotation
        movement.rotation += movement.angular_velocity * time.delta_secs();
    });
}

pub(super) fn physics_transform_extrapolate_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Movement)>,
) {
    query.par_iter_mut().for_each(|(mut transform, movement)| {
        // Rotation
        let future_rotation =
            movement.rotation + movement.angular_velocity * fixed_time.delta_secs();
        let extrapolated_z = movement
            .rotation
            .lerp(future_rotation, fixed_time.delta_secs());
        transform.rotation = Quat::from_rotation_z(extrapolated_z);

        // Translation
        let future_position = movement.position + movement.velocity * fixed_time.delta_secs();
        transform.translation = movement
            .position
            .lerp(future_position, fixed_time.overstep_fraction())
            .extend(0.0);
    });
}
