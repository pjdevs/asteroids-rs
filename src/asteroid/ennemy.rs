use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct EnnemyAssets {
    pub texture: Handle<Image>,
}

// #[derive(Event, Default)]
// pub struct EnnemySpawnedEvent(Vec2);

#[derive(Component)]
pub struct Ennemy {
    velocity: Vec2,
}

pub fn update_timers_system(time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    timer.0.tick(time.delta());
}

pub fn should_spawn_ennemies(timer: Res<SpawnTimer>) -> bool {
    timer.0.just_finished()
}

pub fn spawn_ennemies_system(
    mut commands: Commands,
    ennemy_assets: Res<EnnemyAssets>,
    camera_query: Query<&Camera>,
    // mut spawn_event: EventWriter<EnnemySpawnedEvent>,
) {
    let camera = camera_query.single();
    let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
    let random_speed = rand::random::<f32>() * 0.2 + 1.0;
    let velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position: Vec2 =
        2.0 * half_screen_size * Vec2::new(rand::random::<f32>(), rand::random::<f32>())
            - half_screen_size;

    commands.spawn((
        SpriteBundle {
            texture: ennemy_assets.texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform {
                translation: random_position.extend(0.0),
                ..default()
            },
            ..default()
        },
        Ennemy { velocity },
    ));

    // spawn_event.send(EnnemySpawnedEvent(random_position));
}

// pub fn ennemy_spawn_event_handler_system(mut spawn_event: EventReader<EnnemySpawnedEvent>) {
//     for event in spawn_event.read() {
//         println!("Ennemy spawned at {:?}", event.0);
//     }
// }

pub fn ennemies_movement_system(
    time: Res<Time>,
    mut ennemy_query: Query<(&Ennemy, &mut Transform)>,
) {
    for (ennemy, mut ennemy_transform) in &mut ennemy_query {
        ennemy_transform.translation += ennemy.velocity.extend(0.0);
    }
}

pub fn ennemies_despawn_system(
    mut commands: Commands,
    ennemy_query: Query<(Entity, &Transform), With<Ennemy>>,
    camera_query: Query<&Camera>,
) {
    let camera = camera_query.single();
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);

    ennemy_query
        .iter()
        .for_each(|(ennemy_entity, ennemy_transform)| {
            if ennemy_transform.translation.x.abs() > half_screen_size.x
                || ennemy_transform.translation.y.abs() > half_screen_size.y
            {
                commands.entity(ennemy_entity).despawn();
            }
        });
}
