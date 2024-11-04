mod animation;
mod audio;
mod core;
#[cfg(feature = "dev")]
mod debug;
mod game;
mod input;
mod physics;
mod ui;
mod utils;

use audio::AsteroidAudioPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::DefaultPlugins;
use core::actions::AsteroidAction;
#[cfg(feature = "dev")]
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
    #[allow(unreachable_code)]
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "dev")]
        return Self::dev_plugins();

        Self::default_plugins()
    }
}

impl AsteroidPlugins {
    fn default_plugins() -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins.set(asteroid_window_plugin()))
            .add(AsteroidSetupPlugin)
            .add(AsteroidInputPlugin::<AsteroidAction>::default())
            .add(AsteroidPhysicsPlugin)
            .add(AsteroidBorderPlugin)
            .add(AsteroidProjectilePlugin)
            .add(AsteroidPlayerPlugin)
            .add(AsteroidEnemyPlugin)
            .add(AsteroidGameplayPlugin)
            .add(AsteroidMenuUiPlugin)
            .add(AsteroidGameUiPlugin)
            .add(AsteroidAudioPlugin)
    }

    #[cfg(feature = "dev")]
    fn dev_plugins() -> PluginGroupBuilder {
        Self::default_plugins().add(AsteroidDebugPlugin)
    }
}
