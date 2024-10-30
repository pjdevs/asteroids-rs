use super::{
    border::DespawnBorder,
    gameplay::{CollisionDamager, Health},
    physics::{collision::Collider, movement::Movement},
    states::AsteroidGameState,
    systems::despawn_entities_with,
};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AsteroidProjectile;

#[derive(Bundle, Default)]
pub struct AsteroidProjectileBundle {
    pub projectile: AsteroidProjectile,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: Collider,
    pub border: DespawnBorder,
    pub health: Health,
    pub damager: CollisionDamager,
}

pub struct AsteroidProjectilePlugin;

impl Plugin for AsteroidProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AsteroidGameState::InGame),
            (despawn_entities_with::<AsteroidProjectile>,),
        );
    }
}
