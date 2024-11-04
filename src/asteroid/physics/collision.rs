use std::ops::{BitAnd, BitOr};

use super::obb::Obb2d;
use super::Movement;
use bevy::math::bounding::*;
use bevy::prelude::*;

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

    pub fn transformed_by(&self, movement: &Movement) -> Self {
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

    pub fn scaled(&self, scale: f32) -> Self {
        match self {
            Shape::Aabb(aabb) => Shape::Aabb(aabb.scale_around_center((scale, scale))),
            Shape::Obb(obb) => Shape::Obb(obb.scale_around_center((scale, scale))),
            Shape::Circle(circle) => Shape::Circle(circle.scale_around_center(scale)),
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

#[derive(Event, Clone, Copy)]
pub struct CollisionEvent {
    pub first: Entity,
    pub second: Entity,
}

pub type BitMask = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LayerMask(pub BitMask);

impl LayerMask {
    pub const ALL: LayerMask = LayerMask(BitMask::MAX);
    pub const NONE: LayerMask = LayerMask(BitMask::MIN);
}

impl BitAnd for LayerMask {
    type Output = LayerMask;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        LayerMask(self.0 & rhs.0)
    }
}

impl BitOr for LayerMask {
    type Output = LayerMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        LayerMask(self.0 | rhs.0)
    }
}

#[derive(Component)]
pub struct CollisionLayers {
    members: LayerMask,
    filters: LayerMask,
}

impl CollisionLayers {
    pub fn new(members: LayerMask, filters: LayerMask) -> Self {
        Self { members, filters }
    }
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            members: LayerMask::ALL,
            filters: Default::default(),
        }
    }
}

impl CollisionLayers {
    pub fn interact_with(&self, layers: &CollisionLayers) -> bool {
        self.members & layers.filters != LayerMask::NONE
            && self.filters & layers.members != LayerMask::NONE
    }
}

pub(super) fn physics_collision_system(
    mut events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider, &CollisionLayers, &Movement)>,
) {
    // TODO Implement a general collision system with quadtree, BVH ??
    for [(entity_first, collider_first, layers_first, movement_first), (entity_second, collider_second, layers_second, movement_second)] in
        query.iter_combinations()
    {
        if !collider_first.enabled
            || !collider_second.enabled
            || !layers_first.interact_with(layers_second)
        {
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
