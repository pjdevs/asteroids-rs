mod audio;
mod core;
mod debug;
mod game;
mod input;
mod physics;
mod ui;
mod utils;

use audio::AsteroidAudioPlugin;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};
use core::actions::AsteroidAction;
use debug::AsteroidDebugPlugin;
use game::border::AsteroidBorderPlugin;
use game::enemy::AsteroidEnemyPlugin;
use game::gameplay::AsteroidGameplayPlugin;
use game::player::AsteroidPlayerPlugin;
use game::projectile::AsteroidProjectilePlugin;
use game::setup::AsteroidSetupPlugin;
use input::AsteroidInputPlugin;
use physics::AsteroidPhysicsPlugin;
use ui::game::AsteroidGameUiPlugin;
use ui::menu::AsteroidMenuUiPlugin;
use utils::window::asteroid_window_plugin;

pub struct AsteroidPlugins;

impl PluginGroup for AsteroidPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins.set(asteroid_window_plugin()))
            .add(AsteroidSetupPlugin)
            .add(AsteroidInputPlugin::<AsteroidAction>::default())
            .add(AsteroidPhysicsPlugin)
            .add(AsteroidBorderPlugin)
            .add(AsteroidProjectilePlugin)
            .add(AsteroidPlayerPlugin)
            .add(AsteroidEnemyPlugin {
                enemy_spawn_delay_seconds: 1,
            })
            .add(AsteroidDebugPlugin)
            .add(AsteroidGameplayPlugin)
            .add(AsteroidMenuUiPlugin)
            .add(AsteroidGameUiPlugin)
            .add(AsteroidAudioPlugin)
    }
}
