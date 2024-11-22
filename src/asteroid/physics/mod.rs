pub mod collision;
pub mod movement;
pub mod obb;

use bevy::prelude::*;
use collision::*;
use movement::*;

use super::core::prelude::*;

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(
                FixedPostUpdate,
                (
                    physics_fixed_movement_system
                        .in_set(PhysicsSystem::FixedPostUpdateMovement),
                    physics_collision_system
                        .in_set(PhysicsSystem::FixedPostUpdateCollisionDetection)
                        .after(physics_fixed_movement_system),
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                PostUpdate,
                physics_transform_extrapolate_system
                    .run_if(in_state(GameState::Game))
                    .before(TransformSystem::TransformPropagate)
                    .in_set(PhysicsSystem::PostUpdateExtrapolateTransform),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSystem {
    FixedPostUpdateMovement,
    PostUpdateExtrapolateTransform,
    FixedPostUpdateCollisionDetection,
}

pub mod prelude {
    pub use super::collision::{Collider, CollisionEvent, CollisionLayers, Shape};
    pub use super::movement::Movement;
    pub use super::obb::Obb2d;
}
