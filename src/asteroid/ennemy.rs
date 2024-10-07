use bevy::prelude::*;

use super::{BoxCollider, Movement};

const ENNEMY_SIZE: f32 = 32.0;

#[derive(Resource)]
pub struct EnnemyAssets {
    pub texture: Handle<Image>,
}

impl EnnemyAssets {
    pub fn default(asset_server: &AssetServer) -> Self {
        EnnemyAssets {
            texture: asset_server.load("sprites/ball.png"),
        }
    }
}

// #[derive(Event, Default)]
// pub struct EnnemySpawnedEvent(Vec2);

#[derive(Component)]
pub struct Ennemy;

#[derive(Bundle)]
pub struct EnnemyBundle {
    ennemy: Ennemy,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
}

impl EnnemyBundle {
    pub fn from(ennemy_assets: &EnnemyAssets, position: &Vec2, velocity: &Vec2) -> Self {
        Self {
            ennemy: Ennemy {},
            sprite: SpriteBundle {
                texture: ennemy_assets.texture.clone(),
                ..default()
            },
            movement: Movement {
                position: *position,
                velocity: *velocity,
                ..default()
            },
            collider: BoxCollider {
                size: Vec2::splat(ENNEMY_SIZE),
            },
        }
    }
}

pub fn spawn_ennemies_system(
    mut commands: Commands,
    ennemy_assets: Res<EnnemyAssets>,
    camera_query: Query<&Camera>,
    // mut spawn_event: EventWriter<EnnemySpawnedEvent>,
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

    commands.spawn(EnnemyBundle::from(
        &ennemy_assets,
        &random_position,
        &random_velocity,
    ));

    // spawn_event.send(EnnemySpawnedEvent(random_position));
}

// pub fn ennemy_spawn_event_handler_system(mut spawn_event: EventReader<EnnemySpawnedEvent>) {
//     for event in spawn_event.read() {
//         println!("Ennemy spawned at {:?}", event.0);
//     }
// }
