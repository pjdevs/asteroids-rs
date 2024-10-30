pub mod border;
pub mod enemy;
pub mod gameplay;
pub mod player;
pub mod projectile;
pub mod setup;

pub mod prelude {
    pub use super::{
        border::{DespawnBorder, TunnelBorder},
        enemy::AsteroidEnemy,
        gameplay::{CollisionDamager, Health, Score},
        player::{
            player_exists, spawn_first_player_system, spawn_second_player_system, AsteroidPlayer,
        },
        projectile::AsteroidProjectileBundle,
    };
}
