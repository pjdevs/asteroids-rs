use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use bevy::prelude::*;

pub struct AsteroidScalePlugin;

impl Plugin for AsteroidScalePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AsteroidGameState::Game), setup_scale_hooks);
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
