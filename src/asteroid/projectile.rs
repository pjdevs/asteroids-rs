use super::{
    border::DespawnBorder,
    physics::{BoxCollider, Movement},
};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AsteroidProjectile;

#[derive(Bundle, Default)]
pub struct AsteroidProjectileBundle {
    pub projectile: AsteroidProjectile,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: BoxCollider,
    pub border: DespawnBorder,
}
