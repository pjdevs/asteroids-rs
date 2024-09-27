use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, update_timers_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, spawn_ennemies_system.run_if(should_spawn_ennemies))
        .add_systems(Update, ennemies_movement_system)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn update_timers_system(time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    timer.0.tick(time.delta());
}

fn should_spawn_ennemies(timer: Res<SpawnTimer>) -> bool {
    timer.0.just_finished()
}

fn spawn_ennemies_system(mut commands: Commands, asset_server: Res<AssetServer>, camera_query: Query<&Camera>) {
    let camera = camera_query.single();
    let random_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
    let random_speed = rand::random::<f32>() * 100.0 + 50.0;
    let velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position: Vec2 = 2.0 * half_screen_size * Vec2::new(rand::random::<f32>(), rand::random::<f32>()) - half_screen_size;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball.png"),
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
}

fn ennemies_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&Ennemy, &mut Transform)>,
) {
    for (ennemy, mut ennemy_transform) in &mut player_query {
        ennemy_transform.translation += (ennemy.velocity * time.delta_seconds()).extend(0.0);
    }
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

    player_transform.translation += (input_direction * time.delta_seconds() * player.speed).extend(0.0);
}
