mod asteroid;
mod window;

use asteroid::AsteroidPlugins;
use bevy::prelude::*;
use window::asteroid_window_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(asteroid_window_plugin()))
        .add_plugins(AsteroidPlugins)
        .run();
}
