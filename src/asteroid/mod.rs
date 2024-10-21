mod actions;
mod assets;
mod border;
mod debug;
mod enemy;
mod game;
mod gameplay;
mod input;
mod physics;
mod player;
mod projectile;
mod states;
mod systems;
mod ui;

use actions::AsteroidAction;
use bevy::app::Plugin;
use border::AsteroidBorderPlugin;
use debug::AsteroidDebugPlugin;
use enemy::AsteroidEnemyPlugin;
use game::AsteroidGamePlugin;
use gameplay::AsteroidGameplayPlugin;
use input::AsteroidInputPlugin;
use physics::AsteroidPhysicsPlugin;
use player::AsteroidPlayerPlugin;
use ui::AsteroidUiPlugin;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(AsteroidInputPlugin::<AsteroidAction>::default())
            .add_plugins(AsteroidPhysicsPlugin)
            .add_plugins(AsteroidBorderPlugin)
            .add_plugins(AsteroidPlayerPlugin)
            .add_plugins(AsteroidEnemyPlugin {
                enemy_spawn_delay_seconds: 1,
            })
            .add_plugins(AsteroidDebugPlugin)
            .add_plugins(AsteroidGameplayPlugin)
            .add_plugins(AsteroidUiPlugin)
            .add_plugins(AsteroidGamePlugin);
    }
}
