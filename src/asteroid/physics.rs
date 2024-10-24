use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use core::f32;

pub struct AsteroidPhysicsPlugin;

impl Plugin for AsteroidPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            physics_fixed_movement_system.in_set(AsteroidPhysicsSystem::FixedUpdateMovement),
        )
        .add_systems(
            PostUpdate,
            physics_transform_extrapolate_system
                .before(TransformSystem::TransformPropagate)
                .in_set(AsteroidPhysicsSystem::PostUpdateExtrapolateTransform),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidPhysicsSystem {
    FixedUpdateMovement,
    PostUpdateExtrapolateTransform,
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

#[derive(Component)]
pub struct BoxCollider {
    pub enabled: bool,
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            enabled: true,
            size: Vec2::ONE,
        }
    }
}

impl BoxCollider {
    pub fn obb_2d(&self, movement: &Movement) -> Obb2d {
        Obb2d {
            center: movement.position,
            half_size: self.size / 2.0,
            rotation: movement.rotation.into(),
        }
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

// Obb2d

/// A 2D oriented bounding box
#[derive(Debug, Clone, Copy)]
pub struct Obb2d {
    center: Vec2,
    half_size: Vec2,
    rotation: Rot2,
}

impl Obb2d {
    pub fn new(aabb: Aabb2d, rotation: impl Into<Rot2>) -> Self {
        Self {
            center: aabb.center(),
            half_size: aabb.half_size(),
            rotation: rotation.into(),
        }
    }

    pub fn aabb_2d(&self) -> Aabb2d {
        Rectangle::from_size(self.half_size * 2.0).aabb_2d(self.center, self.rotation)
    }

    /// Project the OBB2d onto a given axis and check if there's an overlap
    fn overlap_on_axis_2d(&self, obb: &Obb2d, axis: Vec2) -> bool {
        let projection1 = Self::project_obb2d(self, axis);
        let projection2 = Self::project_obb2d(obb, axis);

        // Check if projections overlap
        projection1.1 >= projection2.0 && projection2.1 >= projection1.0
    }

    /// Project an OBB2d onto an axis and return the minimum and maximum points
    fn project_obb2d(obb: &Obb2d, axis: Vec2) -> (f32, f32) {
        // Get the corners of the OBB
        let corners = [
            Vec2::new(obb.half_size.x, obb.half_size.y),
            Vec2::new(-obb.half_size.x, obb.half_size.y),
            Vec2::new(obb.half_size.x, -obb.half_size.y),
            Vec2::new(-obb.half_size.x, -obb.half_size.y),
        ];

        // Rotate the corners
        let rotated_corners: Vec<Vec2> = corners
            .iter()
            .map(|corner| obb.center + obb.rotation * *corner)
            .collect();

        // Project the corners onto the axis and find min and max
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for corner in &rotated_corners {
            let projection = axis.dot(*corner);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }
}

impl BoundingVolume for Obb2d {
    type Translation = Vec2;
    type Rotation = Rot2;
    type HalfSize = Vec2;

    fn center(&self) -> Self::Translation {
        self.center
    }

    fn half_size(&self) -> Self::HalfSize {
        self.half_size
    }

    fn visible_area(&self) -> f32 {
        self.half_size.x * self.half_size.y * 4.0
    }

    fn contains(&self, _other: &Self) -> bool {
        todo!()
    }

    fn merge(&self, _other: &Self) -> Self {
        todo!()
    }

    fn grow(&self, amount: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size + amount.into(),
            rotation: self.rotation,
        }
    }

    fn shrink(&self, amount: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size - amount.into(),
            rotation: self.rotation,
        }
    }

    fn scale_around_center(&self, scale: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size * scale.into(),
            rotation: self.rotation,
        }
    }

    fn translate_by(&mut self, translation: impl Into<Self::Translation>) {
        self.center += translation.into()
    }

    fn rotate_by(&mut self, rotation: impl Into<Self::Rotation>) {
        self.rotation = Rot2::radians(self.rotation.as_radians() + rotation.into().as_radians())
    }
}

impl IntersectsVolume<Obb2d> for Obb2d {
    /// Check if two [`Obb2d`]s intersect in 2D using the Separating Axis Theorem (SAT)
    fn intersects(&self, obb: &Obb2d) -> bool {
        // Get the 2 axes from the first OBB (its local x and y axes)
        let axes1 = [
            self.rotation * Vec2::X, // Local x-axis
            self.rotation * Vec2::Y, // Local y-axis
        ];

        // Get the 2 axes from the second OBB (its local x and y axes)
        let axes2 = [
            obb.rotation * Vec2::X, // Local x-axis
            obb.rotation * Vec2::Y, // Local y-axis
        ];

        // Check for separation on each axis
        for axis in axes1.iter().chain(axes2.iter()) {
            if !self.overlap_on_axis_2d(obb, *axis) {
                return false; // Separating axis found, so no intersection
            }
        }

        // If no separating axis is found, the OBBs are intersecting
        true
    }
}
