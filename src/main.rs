use asteroid::AsteroidPlugin;
use bevy::prelude::*;

mod asteroid;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // .disable::<LogPlugin>()
                // .disable::<DiagnosticsPlugin>(),
        )
        .add_plugins(AsteroidPlugin)
        .run();
}
