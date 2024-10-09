use super::{
    border::DespawnBorder,
    physics::{BoxCollider, Movement},
};
use bevy::prelude::*;

// TODO This is too a copy of ennemy

pub struct AsteroidProjectilePlugin {
    pub projectile_size: Vec2,
}

impl Plugin for AsteroidProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_projectile_assets_system(self.projectile_size));
    }
}

fn load_projectile_assets_system(projectile_size: Vec2) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands: Commands, asset_server: Res<AssetServer>| {
        commands.insert_resource(AsteroidProjectileAssets {
            projectile_size,
            texture: asset_server.load("sprites/laser.png"),
        });
    }
}

#[derive(Component, Default)]
pub struct AsteroidProjectile;

// TODO Refactor this one and the ennemy one ?
#[derive(Resource)]
pub struct AsteroidProjectileAssets {
    pub projectile_size: Vec2,
    pub texture: Handle<Image>,
}

#[derive(Bundle, Default)]
pub struct AsteroidProjectileBundle {
    pub projectile: AsteroidProjectile,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: BoxCollider,
    pub border: DespawnBorder,
}
