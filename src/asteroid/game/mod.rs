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
pub mod timed;

pub mod prelude {
    pub use super::{
        border::{DespawnBorder, TunnelBorder},
        damage::{
            DamageSystem, CollisionDamager, Damager, Dead, DespawnOnCollision,
            DespawnOnDead, Health, Invincibility,
        },
        enemy::{Enemy, EnemyAssets},
        gameplay::{PlayerLives, PlayerLivesChanged, Score, ScoreChanged},
        player::{
            spawn_first_player_system, spawn_second_player_system, Player,
            PlayerAssets, PlayerSystem, PlayerShoot, PlayerSpawned, SpawnPlayer,
        },
        projectile::ProjectileBundle,
        scale::Scaled,
        spawner::{Spawner, SpawnerAppExt, SpawnerAsset},
    };
}
