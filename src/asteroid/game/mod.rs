pub mod border;
pub mod enemy;
pub mod gameplay;
pub mod player;
pub mod projectile;
pub mod setup;
pub mod spawner;

pub mod prelude {
    pub use super::{
        border::{KillBorder, TunnelBorder},
        enemy::{AsteroidEnemy, AsteroidEnemySpawner},
        gameplay::{AsteroidGameplaySystem, CollisionDamager, Dead, Health, KillCollision, Score},
        player::{AsteroidPlayerSystem, PlayerShoot},
        projectile::AsteroidProjectileBundle,
        spawner::{AsteroidSpawner, SpawnerAppExt},
    };
}
