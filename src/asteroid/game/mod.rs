pub mod border;
pub mod enemy;
pub mod gameplay;
pub mod player;
pub mod projectile;
pub mod setup;

pub mod prelude {
    pub use super::{
        border::{DeadBorder, TunnelBorder},
        enemy::AsteroidEnemy,
        gameplay::{AsteroidGameplaySystem, CollisionDamager, Dead, Health, Score},
        player::{AsteroidPlayerSystem, PlayerShoot},
        projectile::AsteroidProjectileBundle,
    };
}
