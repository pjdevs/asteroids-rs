use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

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
