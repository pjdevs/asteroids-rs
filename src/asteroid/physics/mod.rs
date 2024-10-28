pub mod collision;
pub mod movement;
pub mod obb;

use std::marker::PhantomData;

use bevy::prelude::*;
use collision::*;
use movement::*;

use super::states::AsteroidGameState;

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
                physics_fixed_movement_system
                    .in_set(AsteroidPhysicsSystem::FixedUpdateMovement)
                    .run_if(in_state(AsteroidGameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                physics_transform_extrapolate_system
                    .run_if(in_state(AsteroidGameState::InGame))
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
            FixedUpdate,
            collision_detection_between::<A, B>
                .run_if(in_state(AsteroidGameState::InGame))
                .after(AsteroidPhysicsSystem::FixedUpdateMovement)
                .in_set(AsteroidPhysicsSystem::FixedUpdateCollisionDetection),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidPhysicsSystem {
    FixedUpdateMovement,
    PostUpdateExtrapolateTransform,
    FixedUpdateCollisionDetection,
}
