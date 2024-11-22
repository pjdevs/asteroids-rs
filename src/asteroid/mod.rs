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

use animation::AnimationPlugin;
use audio::AudioPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::ImagePlugin;
use bevy::DefaultPlugins;
use bevy_trauma_shake::TraumaPlugin;
use core::actions::ShipAction;
#[cfg(feature = "dev")]
use debug::DebugPlugin;
use game::border::BorderPlugin;
use game::damage::DamagePlugin;
use game::effects::EffectsPlugin;
use game::enemy::EnemyPlugin;
use game::gameplay::GameplayPlugin;
use game::player::PlayerPlugin;
use game::projectile::ProjectilePlugin;
use game::scale::ScalePlugin;
use game::setup::SetupPlugin;
use game::ship::ShipPlugin;
use input::InputPlugin;
use physics::PhysicsPlugin;
use ui::game::GameUiPlugin;
use ui::menu::MenuUiPlugin;
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
            .add_group(
                DefaultPlugins
                    .set(asteroid_window_plugin())
                    .set(ImagePlugin::default_nearest()),
            )
            .add(TraumaPlugin)
            .add(SetupPlugin)
            .add(InputPlugin::<ShipAction>::default())
            .add(PhysicsPlugin)
            .add(ScalePlugin)
            .add(AnimationPlugin)
            .add(BorderPlugin)
            .add(ProjectilePlugin)
            .add(ShipPlugin)
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(DamagePlugin)
            .add(GameplayPlugin)
            .add(EffectsPlugin)
            .add(MenuUiPlugin)
            .add(GameUiPlugin)
            .add(AudioPlugin)
    }

    #[cfg(feature = "dev")]
    fn dev_plugins() -> PluginGroupBuilder {
        Self::default_plugins().add(DebugPlugin)
    }
}
