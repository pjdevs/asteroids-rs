pub mod collision;
pub mod movement;
pub mod obb;

use bevy::prelude::*;
use collision::*;
use movement::*;

use super::states::AsteroidGameState;

#[derive(Default)]
pub struct AsteroidPhysicsPlugin;

impl Plugin for AsteroidPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(
                FixedUpdate,
                (
                    physics_fixed_movement_system
                        .in_set(AsteroidPhysicsSystem::FixedUpdateMovement),
                    physics_collision_system
                        .in_set(AsteroidPhysicsSystem::FixedUpdateCollisionDetection)
                        .after(physics_fixed_movement_system),
                )
                    .run_if(in_state(AsteroidGameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                physics_transform_extrapolate_system
                    .run_if(in_state(AsteroidGameState::InGame))
                    .before(TransformSystem::TransformPropagate)
                    .in_set(AsteroidPhysicsSystem::PostUpdateExtrapolateTransform),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidPhysicsSystem {
    FixedUpdateMovement,
    PostUpdateExtrapolateTransform,
    FixedUpdateCollisionDetection,
}
