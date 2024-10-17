use bevy::prelude::*;

use super::{
    actions::AsteroidAction,
    border::TunnelBorder,
    controller::Speed,
    input::{on_gamepad_connection, AxisSide, ButtonMode, InputController, InputMap, InputMapping},
    physics::{BoxCollider, Movement},
    projectile::{AsteroidProjectileAssets, AsteroidProjectileBundle},
};

const PLAYER_SIZE: f32 = 48.0;

pub struct AsteroidPlayerPlugin;

impl Plugin for AsteroidPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_player_system)
            .add_systems(
                Update,
                spawn_second_player_system
                    .run_if(on_gamepad_connection(0).and_then(not(player_exists(2)))),
            )
            .add_systems(Update, player_shoot_system);
    }
}

#[derive(Component, Default)]
pub struct AsteroidPlayer {
    player_id: u64,
}

#[derive(Bundle, Default)]
pub struct AsteroidPlayerBundle {
    player: AsteroidPlayer,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
    border: TunnelBorder,
    speed: Speed,
    controller: InputController<AsteroidAction>,
}

impl AsteroidPlayerBundle {
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.sprite.texture = texture;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.collider.size = size;
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.movement.friction = friction;
        self
    }

    pub fn with_movement_speed(mut self, movement_speed: f32) -> Self {
        self.speed.movement_speed = movement_speed;
        self
    }

    pub fn with_rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.speed.rotation_speed = rotation_speed;
        self
    }

    pub fn with_input_map(mut self, input_map: InputMap<AsteroidAction>) -> Self {
        self.controller = InputController::from_map(input_map);
        self
    }

    pub fn with_id(mut self, player_id: u64) -> Self {
        self.player.player_id = player_id;
        self
    }

    pub fn preset_ship_fast() -> Self {
        AsteroidPlayerBundle::default()
            .with_size(Vec2::splat(PLAYER_SIZE))
            .with_friction(0.03)
            .with_movement_speed(750.0)
            .with_rotation_speed(5.0)
    }

    pub fn preset_ship_slow() -> Self {
        AsteroidPlayerBundle::default()
            .with_size(Vec2::splat(PLAYER_SIZE))
            .with_friction(0.05)
            .with_movement_speed(500.0)
            .with_rotation_speed(4.0)
    }
}

pub fn player_exists(player_id: u64) -> impl Fn(Query<&AsteroidPlayer>) -> bool {
    move |query: Query<&AsteroidPlayer>| query.iter().any(|p| p.player_id == player_id)
}

pub fn spawn_first_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        AsteroidPlayerBundle::preset_ship_fast()
            .with_id(1)
            .with_texture(asset_server.load("sprites/ship_blue.png"))
            .with_input_map(InputMap::default().with_keyboard_mappings()),
    );
}

pub fn spawn_second_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        AsteroidPlayerBundle::preset_ship_slow()
            .with_id(2)
            .with_texture(asset_server.load("sprites/ship_red.png"))
            .with_input_map(InputMap::default().with_gamepad_mappings(0)),
    );
}

fn player_shoot_system(
    mut commands: Commands,
    projectile_assets: Res<AsteroidProjectileAssets>,
    player_query: Query<(&InputController<AsteroidAction>, &Movement), With<AsteroidPlayer>>,
) {
    const PROJECTILE_SPEED: f32 = 600.0;

    for (controller, player_movement) in &player_query {
        if controller.input_action(AsteroidAction::Shoot) {
            commands.spawn(AsteroidProjectileBundle {
                sprite: SpriteBundle {
                    texture: projectile_assets.texture.clone(),
                    ..Default::default()
                },
                movement: Movement {
                    position: player_movement.position,
                    velocity: player_movement.get_direction() * PROJECTILE_SPEED,
                    rotation: player_movement.rotation,
                    ..Default::default()
                },
                collider: BoxCollider {
                    size: projectile_assets.projectile_size,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

impl InputMap<AsteroidAction> {
    fn with_keyboard_mappings(self) -> Self {
        self.with_mapping(
            AsteroidAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::Backward,
            InputMapping::key(KeyCode::ArrowDown, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::TurnLeft,
            InputMapping::key(KeyCode::ArrowLeft, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::TurnRight,
            InputMapping::key(KeyCode::ArrowRight, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::Shoot,
            InputMapping::key(KeyCode::Space, ButtonMode::JustPressed),
        )
        .with_mapping(
            AsteroidAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
    }

    fn with_gamepad_mappings(self, gamepad_id: usize) -> Self {
        self.with_mapping(
            AsteroidAction::Forward,
            InputMapping::button(GamepadButtonType::RightTrigger2, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::Backward,
            InputMapping::button(GamepadButtonType::LeftTrigger2, ButtonMode::Pressed),
        )
        .with_mapping(
            AsteroidAction::TurnLeft,
            InputMapping::axis(GamepadAxisType::LeftStickX, AxisSide::Negative),
        )
        .with_mapping(
            AsteroidAction::TurnRight,
            InputMapping::axis(GamepadAxisType::LeftStickX, AxisSide::Positive),
        )
        .with_mapping(
            AsteroidAction::Shoot,
            InputMapping::button(GamepadButtonType::South, ButtonMode::JustPressed),
        )
        .with_gamepad(Gamepad { id: gamepad_id })
    }
}
