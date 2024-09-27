use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_event::<EnnemySpawnedEvent>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update_timers_system,
                player_movement_system,
                spawn_ennemies_system.run_if(should_spawn_ennemies),
                ennemies_movement_system,
                ennemy_spawn_event_handler_system,
                ennemies_despawn_system,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Ennemy {
    velocity: Vec2,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Event, Default)]
struct EnnemySpawnedEvent(Vec2);

#[derive(Resource)]
struct EnnemyAssets {
    texture: Handle<Image>,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ship.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        },
        Player { speed: 200.0 },
    ));

    commands.insert_resource(EnnemyAssets {
        texture: asset_server.load("sprites/ball.png"),
    });
}

fn update_timers_system(time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    timer.0.tick(time.delta());
}

fn should_spawn_ennemies(timer: Res<SpawnTimer>) -> bool {
    timer.0.just_finished()
}

fn spawn_ennemies_system(
    mut commands: Commands,
    ennemy_assets: Res<EnnemyAssets>,
    camera_query: Query<&Camera>,
    mut spawn_event: EventWriter<EnnemySpawnedEvent>,
) {
    let camera = camera_query.single();
    let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
    let random_speed = rand::random::<f32>() * 100.0 + 50.0;
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

    spawn_event.send(EnnemySpawnedEvent(random_position));
}

fn ennemy_spawn_event_handler_system(mut spawn_event: EventReader<EnnemySpawnedEvent>) {
    for event in spawn_event.read() {
        println!("Ennemy spawned at {:?}", event.0);
    }
}

fn ennemies_movement_system(time: Res<Time>, mut ennemy_query: Query<(&Ennemy, &mut Transform)>) {
    for (ennemy, mut ennemy_transform) in &mut ennemy_query {
        ennemy_transform.translation += (ennemy.velocity * time.delta_seconds()).extend(0.0);
    }
}

fn ennemies_despawn_system(
    mut commands: Commands,
    ennemy_query: Query<(Entity, &Transform), With<Ennemy>>,
    camera_query: Query<&Camera>,
) {
    let camera = camera_query.single();
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);

    let mut ennemy_count = 0;

    ennemy_query
        .iter()
        .for_each(|(ennemy_entity, ennemy_transform)| {
            if ennemy_transform.translation.x.abs() > half_screen_size.x
                || ennemy_transform.translation.y.abs() > half_screen_size.y
            {
                commands.entity(ennemy_entity).despawn();
            } else {
                ennemy_count += 1;
            }
        });

    println!("There is {} ennemies after despawn", ennemy_count);
}

fn player_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut player_transform) = player_query.single_mut();

    let mut input_direction = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        input_direction.y += 1.0;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        input_direction.y -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        input_direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowRight) {
        input_direction.x += 1.0;
    }

    input_direction = input_direction.normalize_or_zero();

    player_transform.translation +=
        (input_direction * time.delta_seconds() * player.speed).extend(0.0);
}
