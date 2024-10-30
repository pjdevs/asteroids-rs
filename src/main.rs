mod asteroid;

use asteroid::AsteroidPlugins;
use bevy::app::App;

fn main() {
    App::new().add_plugins(AsteroidPlugins).run();
}
