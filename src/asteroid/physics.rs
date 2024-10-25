use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use core::f32;
use std::marker::PhantomData;

#[derive(Default)]
pub struct AsteroidPhysicsPlugin {
    collisions_to_check: Vec<Box<dyn AddToApp + Send + Sync>>,
}

impl AsteroidPhysicsPlugin {
    pub fn with_collisions_between<A: Component, B: Component>(mut self) -> Self {
        self.collisions_to_check
            .push(Box::new(EnableCollisionCheck::<A, B> {
                _a: PhantomData,
                _b: PhantomData,
            }));
        self
    }
}

impl Plugin for AsteroidPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(
                FixedUpdate,
                physics_fixed_movement_system.in_set(AsteroidPhysicsSystem::FixedUpdateMovement),
            )
            .add_systems(
                PostUpdate,
                physics_transform_extrapolate_system
                    .before(TransformSystem::TransformPropagate)
                    .in_set(AsteroidPhysicsSystem::PostUpdateExtrapolateTransform),
            );

        for collision in &self.collisions_to_check {
            collision.add_to_app(app);
        }
    }
}

pub struct EnableCollisionCheck<A: Component, B: Component> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

trait AddToApp {
    fn add_to_app(&self, app: &mut App);
}

impl<A: Component, B: Component> AddToApp for EnableCollisionCheck<A, B> {
    fn add_to_app(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection_between::<A, B>.in_set(AsteroidPhysicsSystem::CollisionDetection),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidPhysicsSystem {
    FixedUpdateMovement,
    PostUpdateExtrapolateTransform,
    CollisionDetection,
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

#[derive(Component, Clone, Copy)]
pub struct Collider {
    pub enabled: bool,
    pub shape: Shape,
}

impl Collider {
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            enabled: true,
            shape,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Shape {
    Aabb(Aabb2d),
    Obb(Obb2d),
    Circle(BoundingCircle),
}

impl Shape {
    pub fn intersects(&self, shape: &Shape) -> bool {
        match (self, shape) {
            (Shape::Aabb(aabb1), Shape::Aabb(aabb2)) => aabb1.intersects(aabb2),
            (Shape::Aabb(aabb), Shape::Obb(obb)) => obb.intersects(aabb),
            (Shape::Aabb(aabb), Shape::Circle(circle)) => aabb.intersects(circle),
            (Shape::Obb(obb), Shape::Aabb(aabb)) => obb.intersects(aabb),
            (Shape::Obb(obb1), Shape::Obb(obb2)) => obb1.intersects(obb2),
            (Shape::Obb(obb), Shape::Circle(circle)) => obb.intersects(circle),
            (Shape::Circle(circle), Shape::Aabb(aabb)) => circle.intersects(aabb),
            (Shape::Circle(circle), Shape::Obb(obb)) => obb.intersects(circle),
            (Shape::Circle(circle1), Shape::Circle(circle2)) => circle1.intersects(circle2),
        }
    }

    pub fn transformed_by(&self, transform: Option<&Movement>) -> Self {
        let Some(movement) = transform else {
            return *self;
        };

        match self {
            Shape::Aabb(aabb) => {
                Shape::Aabb(aabb.transformed_by(movement.position, movement.rotation))
            }
            Shape::Obb(obb) => Shape::Obb(obb.transformed_by(movement.position, movement.rotation)),
            Shape::Circle(circle) => {
                Shape::Circle(circle.transformed_by(movement.position, movement.rotation))
            }
        }
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            enabled: true,
            shape: Shape::Aabb(Aabb2d::new(Vec2::ZERO, Vec2::ONE)),
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

#[derive(Event)]
pub struct CollisionEvent {
    pub first: Entity,
    pub second: Entity,
}

fn collision_detection_between<A: Component, B: Component>(
    mut events: EventWriter<CollisionEvent>,
    query_first: Query<(Entity, &Collider, Option<&Movement>), With<A>>,
    query_second: Query<(Entity, &Collider, Option<&Movement>), With<B>>,
) {
    // TODO Implement this with quadtrees directly in physcis plugin
    // TODO Investigate parallel iteration to trigger event
    for (entity_first, collider_first, movement_first) in &query_first {
        if !collider_first.enabled {
            continue;
        }

        for (entity_second, collider_second, movement_second) in &query_second {
            if !collider_second.enabled {
                continue;
            }

            if collider_first
                .shape
                .transformed_by(movement_first)
                .intersects(&collider_second.shape.transformed_by(movement_second))
            {
                events.send(CollisionEvent {
                    first: entity_first,
                    second: entity_second,
                });
            }
        }
    }
}

fn physics_fixed_movement_system(time: Res<Time>, mut query: Query<&mut Movement>) {
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

fn physics_transform_extrapolate_system(
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
    pub center: Vec2,
    pub half_size: Vec2,
    pub rotation: Rot2,
}

impl Obb2d {
    /// Creates a new Obb2d
    pub fn new(center: Vec2, half_size: Vec2, rotation: impl Into<Rot2>) -> Self {
        Self {
            center,
            half_size,
            rotation: rotation.into(),
        }
    }

    /// Project the Obb2d onto a given axis and check if there's an overlap
    fn overlap_on_axis_2d(&self, obb: &Obb2d, axis: Vec2) -> bool {
        let projection1 = Self::project_obb2d(self, axis);
        let projection2 = Self::project_obb2d(obb, axis);

        // Check if projections overlap
        projection1.1 >= projection2.0 && projection2.1 >= projection1.0
    }

    fn corners(&self) -> Vec<Vec2> {
        // Get the corners of the OBB
        let corners = [
            Vec2::new(self.half_size.x, self.half_size.y),
            Vec2::new(-self.half_size.x, self.half_size.y),
            Vec2::new(self.half_size.x, -self.half_size.y),
            Vec2::new(-self.half_size.x, -self.half_size.y),
        ];

        // Rotate the corners
        corners
            .iter()
            .map(|corner| self.center + self.rotation * *corner)
            .collect()
    }

    /// Project an Obb2d onto an axis and return the minimum and maximum points
    fn project_obb2d(obb: &Obb2d, axis: Vec2) -> (f32, f32) {
        // Project the corners onto the axis and find min and max
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for corner in &obb.corners() {
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

    #[inline(always)]
    fn center(&self) -> Self::Translation {
        self.center
    }

    #[inline(always)]
    fn half_size(&self) -> Self::HalfSize {
        self.half_size
    }

    #[inline(always)]
    fn visible_area(&self) -> f32 {
        self.half_size.x * self.half_size.y * 4.0
    }

    fn contains(&self, _other: &Self) -> bool {
        todo!()
    }

    fn merge(&self, _other: &Self) -> Self {
        todo!()
    }

    #[inline(always)]
    fn grow(&self, amount: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size + amount.into(),
            rotation: self.rotation,
        }
    }

    #[inline(always)]
    fn shrink(&self, amount: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size - amount.into(),
            rotation: self.rotation,
        }
    }

    #[inline(always)]
    fn scale_around_center(&self, scale: impl Into<Self::HalfSize>) -> Self {
        Self {
            center: self.center,
            half_size: self.half_size * scale.into(),
            rotation: self.rotation,
        }
    }

    #[inline(always)]
    fn translate_by(&mut self, translation: impl Into<Self::Translation>) {
        self.center += translation.into()
    }

    #[inline(always)]
    fn rotate_by(&mut self, rotation: impl Into<Self::Rotation>) {
        self.rotation = self.rotation * rotation.into()
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

impl IntersectsVolume<Aabb2d> for Obb2d {
    fn intersects(&self, aabb: &Aabb2d) -> bool {
        self.intersects(&Obb2d::new(aabb.center(), aabb.half_size(), 0.0))
    }
}

impl IntersectsVolume<BoundingCircle> for Obb2d {
    fn intersects(&self, circle: &BoundingCircle) -> bool {
        // Step 1: Rotate the circle's center into the OBB's local space
        let local_circle_center = self.rotation * (circle.center - self.center);

        // Step 2: Clamp the local circle center to the OBB's extents
        let clamped = Vec2::new(
            local_circle_center
                .x
                .clamp(-self.half_size.x, self.half_size.x),
            local_circle_center
                .y
                .clamp(-self.half_size.y, self.half_size.y),
        );

        // Step 3: Check if the distance between the clamped point and the local circle center is less than the radius
        (local_circle_center - clamped).length_squared() <= circle.radius() * circle.radius()
    }
}

impl Bounded2d for Obb2d {
    fn aabb_2d(&self, translation: Vec2, rotation: impl Into<Rot2>) -> Aabb2d {
        Rectangle::new(self.half_size.x * 2.0, self.half_size.y * 2.0)
            .aabb_2d(self.center + translation, rotation.into() * self.rotation)
    }

    fn bounding_circle(
        &self,
        translation: Vec2,
        rotation: impl Into<Rot2>,
    ) -> bevy::math::bounding::BoundingCircle {
        self.aabb_2d(translation, rotation).bounding_circle()
    }
}
