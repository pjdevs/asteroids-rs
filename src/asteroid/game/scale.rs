use crate::asteroid::physics::prelude::*;
use bevy::prelude::*;

use super::prelude::Health;

// TODO Use change detection instead ?

pub struct AsteroidScalePlugin;

impl Plugin for AsteroidScalePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_scale_hooks);
    }
}

#[derive(Component)]
pub struct AsteroidScaled {
    pub scale: f32,
}

impl AsteroidScaled {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl Default for AsteroidScaled {
    fn default() -> Self {
        Self::new(1.0)
    }
}

fn setup_scale_hooks(world: &mut World) {
    world
        .register_component_hooks::<AsteroidScaled>()
        .on_add(|mut world, entity, _| {
            let scale = if let Some(component) = world.get::<AsteroidScaled>(entity) {
                component.scale
            } else {
                1.0
            };

            // TODO Handle this elsewhere as this is more gameplay?
            if let Some(mut health) = world.get_mut::<Health>(entity) {
                let max_health = health.get_max_health() as f32;
                health.set_max_health(max_health.lerp(200.0, scale - 1.0 + 0.2) as i32);
            };

            if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
                if let Some(size) = sprite.custom_size {
                    sprite.custom_size = Some(size * scale);
                }
            };

            if let Some(mut collider) = world.get_mut::<Collider>(entity) {
                collider.shape = collider.shape.scaled(scale);
            };
        });
}
