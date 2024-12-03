use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    Sprite,
    Movement,
    Collider,
    CollisionLayers,
    DespawnBorder,
    DespawnOnCollision,
    CollisionDamager
)]
pub struct Projectile;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Game),
            (despawn_entities_with::<Projectile>,),
        );
    }
}
