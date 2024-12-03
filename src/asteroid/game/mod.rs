pub mod border;
pub mod damage;
pub mod effects;
pub mod enemy;
pub mod gameplay;
pub mod player;
pub mod projectile;
pub mod scale;
pub mod setup;
pub mod ship;
pub mod spawner;
pub mod timed;

pub mod prelude {
    pub use super::{
        border::{DespawnBorder, TunnelBorder},
        damage::{
            CollisionDamager, DamageSystem, Damager, Dead, DespawnOnCollision, DespawnOnDead,
            Health, Invincibility,
        },
        enemy::{Enemy, EnemyAssets},
        gameplay::{PlayerLives, PlayerLivesChanged, Score, ScoreChanged},
        player::{
            player_exists, spawn_first_player_system, spawn_second_player_system, Player,
            PlayerAssets, PlayerSpawned, PlayerSystem, SpawnPlayer,
        },
        projectile::Projectile,
        scale::Scaled,
        ship::{ShipMovement, ShipShoot, ShipSystem, ShootEvent},
        spawner::{Spawner, SpawnerAppExt, SpawnerAsset},
        timed::{TimedAppExt, TimedEntityCommandsExt},
    };
}
