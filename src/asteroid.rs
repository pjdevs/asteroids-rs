mod border;
mod debug;
mod ennemy;
mod gameplay;
mod input;
mod physics;
mod player;
mod projectile;

use bevy::{app::Plugin, math::Vec2};
use border::AsteroidBorderPlugin;
use debug::AsteroidDebugPlugin;
use ennemy::AsteroidEnnemyPlugin;
use gameplay::AsteroidGameplayPlugin;
use input::AsteroidInputPlugin;
use physics::AsteroidPhysicsPlugin;
use player::AsteroidPlayerPlugin;
use projectile::AsteroidProjectilePlugin;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(AsteroidInputPlugin)
            .add_plugins(AsteroidPhysicsPlugin)
            .add_plugins(AsteroidBorderPlugin)
            .add_plugins(AsteroidPlayerPlugin)
            .add_plugins(AsteroidEnnemyPlugin {
                ennemy_size: Vec2::splat(48.0),
                ennemy_spawn_delay_seconds: 3,
            })
            .add_plugins(AsteroidProjectilePlugin {
                projectile_size: Vec2::splat(24.0),
            })
            .add_plugins(AsteroidDebugPlugin)
            .add_plugins(AsteroidGameplayPlugin);
    }
}
