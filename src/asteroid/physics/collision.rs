use std::sync::Mutex;

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

#[derive(Event, Clone, Copy)]
pub struct CollisionEvent {
    pub first: Entity,
    pub second: Entity,
}

pub(super) fn collision_detection_between<A: Component, B: Component>(
    events: EventWriter<CollisionEvent>,
    query_first: Query<(Entity, &Collider, Option<&Movement>), With<A>>,
    query_second: Query<(Entity, &Collider, Option<&Movement>), With<B>>,
) {
    let events_mutex = Mutex::new(events);

    // TODO Implement a general collision system with quadtree, BVH ??
    for (entity_first, collider_first, movement_first) in &query_first {
        if !collider_first.enabled {
            continue;
        }

        query_second
            .par_iter()
            .for_each(|(entity_second, collider_second, movement_second)| {
                if entity_first != entity_second && collider_second.enabled {
                    if collider_first
                        .shape
                        .transformed_by(movement_first)
                        .intersects(&collider_second.shape.transformed_by(movement_second))
                    {
                        match events_mutex.lock() {
                            Ok(mut events) => {
                                events.send(CollisionEvent {
                                    first: entity_first,
                                    second: entity_second,
                                });
                            }
                            Err(err) => {
                                error!("Error locking event reader for collisions: {}", err);
                            }
                        }
                    }
                }
            });
    }
}
