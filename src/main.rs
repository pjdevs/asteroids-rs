mod asteroid;

use asteroid::AsteroidPlugins;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(AsteroidPlugins)
        .run();
}
