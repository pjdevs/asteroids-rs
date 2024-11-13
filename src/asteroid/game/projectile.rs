use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AsteroidProjectile;

#[derive(Bundle, Default)]
pub struct AsteroidProjectileBundle {
    pub projectile: AsteroidProjectile,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: Collider,
    pub layers: CollisionLayers,
    pub border: DespawnBorder,
    pub kill_collision: DespawnCollision,
    pub damager: CollisionDamager,
}

pub struct AsteroidProjectilePlugin;

impl Plugin for AsteroidProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AsteroidGameState::Game),
            (despawn_entities_with::<AsteroidProjectile>,),
        );
    }
}
