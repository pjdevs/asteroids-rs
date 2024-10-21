use super::{
    border::DespawnBorder,
    physics::{BoxCollider, Movement},
};
use bevy::prelude::*;

// TODO This is too a copy of enemy (still relevant?)

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

// TODO Find a way to factorize this and player etc bundles impl
impl AsteroidProjectileBundle {
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.sprite.texture = texture;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.collider.size = size;
        self
    }
}
