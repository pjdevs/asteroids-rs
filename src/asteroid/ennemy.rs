use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::asteroid::physics::BoxCollider;

use super::{border::TunnelBorder, physics::Movement};

pub struct AsteroidEnnemyPlugin {
    pub ennemy_size: Vec2,
    pub ennemy_spawn_delay_seconds: u64,
}

impl Plugin for AsteroidEnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnnemySpawnedEvent>()
            .add_systems(Startup, load_ennemy_assets_system(self.ennemy_size))
            .add_systems(
                Update,
                spawn_ennemies_system.run_if(on_timer(Duration::from_secs(
                    self.ennemy_spawn_delay_seconds,
                ))),
            );
    }
}

fn load_ennemy_assets_system(ennemy_size: Vec2) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands: Commands, asset_server: Res<AssetServer>| {
        commands.insert_resource(AsteroidEnnemyAssets {
            ennemy_size,
            texture: asset_server.load("sprites/ball.png"),
        });
    }
}

#[derive(Resource)]
pub struct AsteroidEnnemyAssets {
    pub ennemy_size: Vec2,
    pub texture: Handle<Image>,
}

#[derive(Event, Default)]
pub struct EnnemySpawnedEvent(Vec2);

#[derive(Component, Default)]
pub struct AsteroidEnnemy;

#[derive(Bundle, Default)]
pub struct AsteroidEnnemyBundle {
    ennemy: AsteroidEnnemy,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
    border: TunnelBorder,
}

impl AsteroidEnnemyBundle {
    pub fn from(ennemy_assets: &AsteroidEnnemyAssets, position: &Vec2, velocity: &Vec2) -> Self {
        Self {
            ennemy: AsteroidEnnemy {},
            sprite: SpriteBundle {
                texture: ennemy_assets.texture.clone(),
                ..Default::default()
            },
            movement: Movement {
                position: *position,
                velocity: *velocity,
                ..Default::default()
            },
            collider: BoxCollider {
                size: ennemy_assets.ennemy_size,
            },
            ..Default::default()
        }
    }
}

// TODO Expose min max speed angle etc

pub fn spawn_ennemies_system(
    mut commands: Commands,
    ennemy_assets: Res<AsteroidEnnemyAssets>,
    camera_query: Query<&Camera>,
    mut spawn_event: EventWriter<EnnemySpawnedEvent>,
) {
    let camera = camera_query.single();
    let random_angle = rand::random::<f32>() * std::f32::consts::PI * 1.99 + 0.1;
    let random_speed = rand::random::<f32>() * 100.0 + 50.0;
    let random_velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position: Vec2 = 2.0
        * half_screen_size
        * Vec2::new(rand::random::<f32>().round(), rand::random::<f32>().round())
        - half_screen_size;

    commands.spawn(AsteroidEnnemyBundle::from(
        &ennemy_assets,
        &random_position,
        &random_velocity,
    ));

    spawn_event.send(EnnemySpawnedEvent(random_position));
}
