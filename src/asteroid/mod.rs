mod actions;
mod assets;
mod border;
mod debug;
mod enemy;
mod game;
mod gameplay;
mod input;
mod macro_helpers;
mod physics;
mod player;
mod projectile;
mod states;
mod systems;
mod ui;
mod window;

use actions::AsteroidAction;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};
use border::AsteroidBorderPlugin;
use debug::AsteroidDebugPlugin;
use enemy::AsteroidEnemyPlugin;
use game::AsteroidGamePlugin;
use gameplay::{AsteroidGameplayPlugin, CollisionDamager, Health};
use input::AsteroidInputPlugin;
use physics::AsteroidPhysicsPlugin;
use player::AsteroidPlayerPlugin;
use projectile::AsteroidProjectilePlugin;
use ui::game::AsteroidGameUiPlugin;
use ui::menu::AsteroidMenuUiPlugin;
use window::asteroid_window_plugin;

pub struct AsteroidPlugins;

impl PluginGroup for AsteroidPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins.set(asteroid_window_plugin()))
            .add(AsteroidGamePlugin)
            .add(AsteroidInputPlugin::<AsteroidAction>::default())
            .add(
                AsteroidPhysicsPlugin::default()
                    .with_collisions_between::<CollisionDamager, Health>(),
            )
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
    }
}
