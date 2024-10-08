mod debug;
mod ennemy;
mod gameplay;
mod physics;
mod player;

use bevy::app::Plugin;
use debug::*;
use ennemy::*;
use gameplay::*;
use physics::*;
use player::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(AsteroidPhysicsPlugin)
            .add_plugins(AsteroidPlayerPlugin)
            .add_plugins(AsteroidEnnemyPlugin::default())
            .add_plugins(AsteroidDebugPlugin)
            .add_plugins(AsteroidGameplayPlugin);
    }
}
