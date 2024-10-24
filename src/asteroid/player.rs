use super::{
    actions::AsteroidAction,
    assets::SizeAsset,
    border::TunnelBorder,
    input::{AxisSide, ButtonMode, InputController, InputMap, InputMapping},
    physics::{BoxCollider, Movement},
    projectile::AsteroidProjectileBundle,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

// TODO Refactor all behaviors in components (Ship, Shoot, ..)

// Plugin

pub struct AsteroidPlayerPlugin;

impl Plugin for AsteroidPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_move_system, player_shoot_system)
                .in_set(AsteroidPlayerSystem::UpdatePlayerActions),
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct AsteroidPlayerAssets {
    #[asset(key = "player.one_texture")]
    pub player_one_texture: Handle<Image>,

    #[asset(key = "player.two_texture")]
    pub player_two_texture: Handle<Image>,

    #[asset(key = "player.projectile.texture")]
    pub projectile_texture: Handle<Image>,

    #[asset(path = "player.size.ron")]
    pub player_size: Handle<SizeAsset>,
}

// Components

#[derive(Component, Default)]
pub struct AsteroidPlayer {
    player_id: u64,
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Bundle, Default)]
pub struct AsteroidPlayerBundle {
    player: AsteroidPlayer,
    sprite: SpriteBundle,
    movement: Movement,
    collider: BoxCollider,
    border: TunnelBorder,
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
        self.player.movement_speed = movement_speed;
        self
    }

    pub fn with_rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.player.rotation_speed = rotation_speed;
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
            .with_friction(0.03)
            .with_movement_speed(750.0)
            .with_rotation_speed(5.0)
    }

    pub fn preset_ship_slow() -> Self {
        AsteroidPlayerBundle::default()
            .with_friction(0.05)
            .with_movement_speed(500.0)
            .with_rotation_speed(4.0)
    }
}

// Conditions

pub fn player_exists(player_id: u64) -> impl Fn(Query<&AsteroidPlayer>) -> bool {
    move |query: Query<&AsteroidPlayer>| query.iter().any(|p| p.player_id == player_id)
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidPlayerSystem {
    UpdatePlayerActions,
}

pub fn spawn_first_player_system(
    mut commands: Commands,
    sizes: Res<Assets<SizeAsset>>,
    assets: Res<AsteroidPlayerAssets>,
) {
    commands.spawn(
        AsteroidPlayerBundle::preset_ship_fast()
            .with_id(1)
            .with_size(
                sizes
                    .get(&assets.player_size)
                    .expect("Cannot find player size assset")
                    .collider_size,
            )
            .with_texture(assets.player_one_texture.clone())
            .with_input_map(InputMap::default().with_keyboard_mappings()),
    );
}

pub fn spawn_second_player_system(
    mut commands: Commands,
    sizes: Res<Assets<SizeAsset>>,
    assets: Res<AsteroidPlayerAssets>,
) {
    commands.spawn(
        AsteroidPlayerBundle::preset_ship_slow()
            .with_id(2)
            .with_size(
                sizes
                    .get(&assets.player_size)
                    .expect("Cannot find player size assset")
                    .collider_size,
            )
            .with_texture(assets.player_two_texture.clone())
            .with_input_map(InputMap::default().with_gamepad_mappings(0)),
    );
}

fn player_shoot_system(
    mut commands: Commands,
    assets: Res<AsteroidPlayerAssets>,
    player_query: Query<(&InputController<AsteroidAction>, &Movement), With<AsteroidPlayer>>,
) {
    const PROJECTILE_SPEED: f32 = 600.0;

    for (controller, player_movement) in &player_query {
        if controller.input_action(AsteroidAction::Shoot) {
            commands.spawn(AsteroidProjectileBundle {
                sprite: SpriteBundle {
                    texture: assets.projectile_texture.clone(),
                    ..Default::default()
                },
                movement: Movement {
                    position: player_movement.position,
                    velocity: player_movement.get_direction() * PROJECTILE_SPEED,
                    rotation: player_movement.rotation,
                    ..Default::default()
                },
                collider: BoxCollider {
                    size: Vec2::new(16.0, 24.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn player_move_system(
    mut query: Query<(
        &mut Movement,
        &AsteroidPlayer,
        &InputController<AsteroidAction>,
    )>,
) {
    for (mut movement, player, controller) in &mut query {
        let mut input_direction = Vec2::ZERO;

        if controller.input_action(AsteroidAction::Forward) {
            input_direction.y += 1.0;
        }

        if controller.input_action(AsteroidAction::Backward) {
            input_direction.y -= 1.0;
        }

        if controller.input_action(AsteroidAction::TurnLeft) {
            input_direction.x -= 1.0;
        }

        if controller.input_action(AsteroidAction::TurnRight) {
            input_direction.x += 1.0;
        }

        // Rotation
        movement.angular_velocity = -input_direction.x * player.rotation_speed;

        // Translation
        movement.acceleration =
            movement.get_direction() * player.movement_speed * input_direction.y;
    }
}

// Input maps

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
