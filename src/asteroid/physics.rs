use bevy::prelude::*;
use core::f32;

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

#[derive(Component)]
pub struct BoxCollider {
    pub size: Vec2,
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

pub fn physics_fixed_system(time: Res<Time>, mut query: Query<&mut Movement>) {
    for mut movement in &mut query {
        let movement = &mut *movement;

        // Translation
        movement.velocity +=
            movement.acceleration * time.delta_seconds() - movement.friction * movement.velocity;
        movement.velocity = movement.velocity.clamp_length_max(movement.max_speed);
        movement.position += movement.velocity * time.delta_seconds();

        // Rotation
        movement.rotation += movement.angular_velocity * time.delta_seconds();
    }
}

pub fn physics_transform_extrapolate_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Movement)>,
) {
    for (mut transform, movement) in &mut query {
        // Rotation
        let future_rotation =
            movement.rotation + movement.angular_velocity * fixed_time.delta_seconds();
        let extrapolated_z = movement
            .rotation
            .lerp(future_rotation, fixed_time.delta_seconds());
        transform.rotation = Quat::from_rotation_z(extrapolated_z);

        // Translation
        let future_position = movement.position + movement.velocity * fixed_time.delta_seconds();
        transform.translation = movement
            .position
            .lerp(future_position, fixed_time.overstep_fraction())
            .extend(0.0);
    }
}

pub fn border_system(mut ennemy_query: Query<&mut Movement>, camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);

    ennemy_query.iter_mut().for_each(|mut ennemy_movement| {
        if ennemy_movement.position.x.abs() > half_screen_size.x + 32.0 {
            ennemy_movement.position.x *= -1.0;
        }

        if ennemy_movement.position.y.abs() > half_screen_size.y + 32.0 {
            ennemy_movement.position.y *= -1.0;
        }
    });
}
