pub mod border;
pub mod damage;
pub mod effects;
pub mod enemy;
pub mod gameplay;
pub mod player;
pub mod projectile;
pub mod scale;
pub mod setup;
pub mod spawner;

pub mod prelude {
    pub use super::{
        border::{KillBorder, TunnelBorder},
        damage::{
            AsteroidDamageSystem, CollisionDamager, Dead, DespawnIfDead, Health, KillCollision,
        },
        enemy::{AsteroidEnemy, AsteroidEnemySpawner},
        gameplay::{PlayerLives, PlayerLivesChanged, Score, ScoreChanged},
        player::{
            spawn_first_player_system, spawn_second_player_system, AsteroidPlayer,
            AsteroidPlayerAssets, AsteroidPlayerSystem, PlayerShoot, PlayerSpawned, SpawnPlayer,
        },
        projectile::AsteroidProjectileBundle,
        scale::AsteroidScaled,
        spawner::{AsteroidSpawner, SpawnerAppExt},
    };
}
