use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingVolume},
    prelude::*,
};
use core::f32;
use std::f32::consts::PI;

pub struct AsteroidPhysicsPlugin;

impl Plugin for AsteroidPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, physics_fixed_movement_system)
            .add_systems(
                PostUpdate,
                physics_transform_extrapolate_system.before(TransformSystem::TransformPropagate),
            );
    }
}

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

#[derive(Component, Default)]
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

pub fn physics_fixed_movement_system(time: Res<Time>, mut query: Query<&mut Movement>) {
    query.par_iter_mut().for_each(|mut movement| {
        let movement = &mut *movement;

        // Translation
        movement.velocity +=
            movement.acceleration * time.delta_seconds() - movement.friction * movement.velocity;
        movement.velocity = movement.velocity.clamp_length_max(movement.max_speed);
        movement.position += movement.velocity * time.delta_seconds();

        // Rotation
        movement.rotation += movement.angular_velocity * time.delta_seconds();
    });
}

pub fn physics_transform_extrapolate_system(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Movement)>,
) {
    query.par_iter_mut().for_each(|(mut transform, movement)| {
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
    });
}

pub fn aabb_from(movement: &Movement, collider: &BoxCollider) -> Aabb2d {
    Rectangle::from_size(collider.size).aabb_2d(movement.position, movement.rotation)
}
