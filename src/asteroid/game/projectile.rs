use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: Collider,
    pub layers: CollisionLayers,
    pub border: DespawnBorder,
    pub kill_collision: DespawnOnCollision,
    pub damager: CollisionDamager,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Game),
            (despawn_entities_with::<Projectile>,),
        );
    }
}
