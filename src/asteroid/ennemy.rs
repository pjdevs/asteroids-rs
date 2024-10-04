use bevy::prelude::*;

use super::Movement;

#[derive(Resource)]
pub struct EnnemyAssets {
    pub texture: Handle<Image>,
}

// #[derive(Event, Default)]
// pub struct EnnemySpawnedEvent(Vec2);

#[derive(Component)]
pub struct Ennemy;

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

    commands.spawn((
        SpriteBundle {
            texture: ennemy_assets.texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        },
        Ennemy {},
        Movement {
            position: random_position,
            velocity: random_velocity,
            ..default()
        },
    ));

    // spawn_event.send(EnnemySpawnedEvent(random_position));
}

// pub fn ennemy_spawn_event_handler_system(mut spawn_event: EventReader<EnnemySpawnedEvent>) {
//     for event in spawn_event.read() {
//         println!("Ennemy spawned at {:?}", event.0);
//     }
// }

pub fn ennemies_border_system(
    mut ennemy_query: Query<&mut Movement, With<Ennemy>>,
    camera_query: Query<&Camera>,
) {
    let camera = camera_query.single();
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);

    ennemy_query.iter_mut().for_each(|mut ennemy_movement| {
        if ennemy_movement.position.x.abs() > half_screen_size.x + 32.0 {
            ennemy_movement.position.x *= -1.0;
        }

        if ennemy_movement.position.y.abs() > half_screen_size.y + 32.0 {
            ennemy_movement.position.y *= -1.0;
        }
    });
}
