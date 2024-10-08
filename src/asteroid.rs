mod debug;
mod ennemy;
mod gameplay;
mod input;
mod physics;
mod player;

use bevy::app::Plugin;
use debug::AsteroidDebugPlugin;
use ennemy::AsteroidEnnemyPlugin;
use gameplay::AsteroidGameplayPlugin;
use input::AsteroidInputPlugin;
use physics::AsteroidPhysicsPlugin;
use player::AsteroidPlayerPlugin;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(AsteroidInputPlugin)
            .add_plugins(AsteroidPhysicsPlugin)
            .add_plugins(AsteroidPlayerPlugin)
            .add_plugins(AsteroidEnnemyPlugin::default())
            .add_plugins(AsteroidDebugPlugin)
            .add_plugins(AsteroidGameplayPlugin);
    }
}
