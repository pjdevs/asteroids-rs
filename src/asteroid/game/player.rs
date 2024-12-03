use crate::asset;
use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::input::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// Plugin

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Game),
            (
                remove_resource::<PlayerAssets>,
                despawn_entities_with::<Player>,
            ),
        )
        .add_systems(
            Update,
            (player_move_system, player_shoot_system)
                .run_if(in_state(GameState::Game))
                .before(ShipSystem::UpdateShips)
                .in_set(PlayerSystem::UpdatePlayerActions),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::GameLoading).load_collection::<PlayerAssets>(),
        );
    }
}

// Assets

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(key = "player.one.texture")]
    pub player_one_texture: Handle<Image>,

    #[asset(key = "player.two.texture")]
    pub player_two_texture: Handle<Image>,

    #[asset(key = "player.projectile.texture")]
    pub player_projectile_texture: Handle<Image>,

    #[asset(path = "player.size.ron")]
    pub player_size: Handle<SizeAsset>,

    #[asset(path = "player.projectile.size.ron")]
    pub player_projectile_size: Handle<SizeAsset>,
}

impl PlayerAssets {
    pub fn get_texture_by_player_id(&self, player_id: &u64) -> Option<Handle<Image>> {
        match player_id {
            1 => Some(self.player_one_texture.clone_weak()),
            2 => Some(self.player_two_texture.clone_weak()),
            _ => None,
        }
    }
}

// Events

#[derive(Event)]
pub struct PlayerSpawned;

// Components

#[derive(Component, Default)]
pub struct Player {
    pub player_id: u64,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    movement: Movement,
    collider: Collider,
    layers: CollisionLayers,
    border: TunnelBorder,
    ship: ShipMovement,
    shoot: ShipShoot,
    input_map: InputMap<ShipAction>,
    health: Health,
    despawn: DespawnOnDead,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Default::default(),
            sprite: Default::default(),
            movement: Default::default(),
            collider: Default::default(),
            layers: CollisionLayers::new(layers::PLAYER_MASK, layers::ENEMY_MASK),
            border: Default::default(),
            ship: Default::default(),
            shoot: Default::default(),
            input_map: Default::default(),
            health: Default::default(),
            despawn: Default::default(),
        }
    }
}

impl PlayerBundle {
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.sprite.image = texture;
        self
    }

    pub fn with_size(mut self, size: &SizeAsset) -> Self {
        self.sprite.custom_size = Some(size.sprite_size);
        self.collider = Collider::from_shape(Shape::Obb(Obb2d::new(
            Vec2::ZERO,
            size.collider_size / 2.0,
            0.0,
        )));
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.movement.friction = friction;
        self
    }

    pub fn with_movement_speed(mut self, movement_speed: f32) -> Self {
        self.ship.movement_speed = movement_speed;
        self
    }

    pub fn with_rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.ship.rotation_speed = rotation_speed;
        self
    }

    pub fn with_projectile_texture(mut self, projectile_texture: Handle<Image>) -> Self {
        self.shoot.projectile_texture = projectile_texture;
        self
    }

    pub fn with_projectile_size(mut self, projectile_size: Handle<SizeAsset>) -> Self {
        self.shoot.projectile_size = projectile_size;
        self
    }

    pub fn with_input_map(mut self, input_map: InputMap<ShipAction>) -> Self {
        self.input_map = input_map;
        self
    }

    pub fn with_id(mut self, player_id: u64) -> Self {
        self.player.player_id = player_id;
        self
    }

    pub fn preset_ship_fast() -> Self {
        PlayerBundle::default()
            .with_friction(0.03)
            .with_movement_speed(750.0)
            .with_rotation_speed(5.0)
    }

    pub fn preset_ship_slow() -> Self {
        PlayerBundle::default()
            .with_friction(0.05)
            .with_movement_speed(500.0)
            .with_rotation_speed(4.0)
    }
}

// Conditions

pub fn player_exists(player_id: u64) -> impl Fn(Query<&Player>) -> bool {
    move |query: Query<&Player>| query.iter().any(|p| p.player_id == player_id)
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum PlayerSystem {
    UpdatePlayerActions,
}

pub fn spawn_first_player_system(mut commands: Commands) {
    commands.queue(SpawnPlayer::from_id(1));
}

pub fn spawn_second_player_system(mut commands: Commands, query: Query<Entity, With<Gamepad>>) {
    commands.queue(SpawnPlayer::from_id_gamepad(2, query.iter().next()));
}

fn first_player_bundle(sizes: &Assets<SizeAsset>, assets: &PlayerAssets) -> PlayerBundle {
    PlayerBundle::preset_ship_fast()
        .with_id(1)
        .with_texture(assets.player_one_texture.clone())
        .with_size(asset!(sizes, &assets.player_size))
        .with_projectile_texture(assets.player_projectile_texture.clone_weak())
        .with_projectile_size(assets.player_projectile_size.clone_weak())
        .with_input_map(InputMap::default().with_keyboard_mappings())
}

fn second_player_bundle(
    sizes: &Assets<SizeAsset>,
    assets: &PlayerAssets,
    gamepad: Option<Entity>,
) -> PlayerBundle {
    PlayerBundle::preset_ship_slow()
        .with_id(2)
        .with_texture(assets.player_two_texture.clone())
        .with_size(asset!(sizes, &assets.player_size))
        .with_projectile_texture(assets.player_projectile_texture.clone_weak())
        .with_projectile_size(assets.player_projectile_size.clone_weak())
        .with_input_map(
            InputMap::default()
                .with_gamepad_mappings()
                .with_gamepad(gamepad),
        )
}

pub struct SpawnPlayer {
    player_id: u64,
    associated_gamepad: Option<Entity>,
}

impl SpawnPlayer {
    pub fn from_id(player_id: u64) -> Self {
        Self {
            player_id,
            associated_gamepad: None,
        }
    }

    pub fn from_id_gamepad(player_id: u64, gamepad: Option<Entity>) -> Self {
        Self {
            player_id,
            associated_gamepad: gamepad,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        let player_entity = {
            let sizes = world
                .get_resource::<Assets<SizeAsset>>()
                .expect("Size assets must exist to spawn player");
            let assets = world
                .get_resource::<PlayerAssets>()
                .expect("Player assets must exist to spawn player");

            match self.player_id {
                1 => world.spawn(first_player_bundle(sizes, assets)).id(),
                2 => world
                    .spawn(second_player_bundle(sizes, assets, self.associated_gamepad))
                    .id(),
                _ => return,
            }
        };

        #[cfg(feature = "dev")]
        world
            .entity_mut(player_entity)
            .insert(Name::new(format!("Player {}", self.player_id)));
        world.trigger_targets(PlayerSpawned, player_entity);
    }
}

fn player_shoot_system(
    mut player_query: Query<(&InputMap<ShipAction>, &mut ShipShoot), With<Player>>,
) {
    for (controller, mut shoot) in &mut player_query {
        shoot.shoot = controller.input_action(ShipAction::Shoot);
    }
}

fn player_move_system(mut query: Query<(&InputMap<ShipAction>, &mut ShipMovement)>) {
    for (controller, mut ship) in &mut query {
        ship.direction = Vec2::ZERO;

        if controller.input_action(ShipAction::Forward) {
            ship.direction.y += 1.0;
        }

        if controller.input_action(ShipAction::Backward) {
            ship.direction.y -= 1.0;
        }

        if controller.input_action(ShipAction::TurnLeft) {
            ship.direction.x -= 1.0;
        }

        if controller.input_action(ShipAction::TurnRight) {
            ship.direction.x += 1.0;
        }
    }
}

// Input maps

impl InputMap<ShipAction> {
    fn with_keyboard_mappings(self) -> Self {
        self.with_mapping(
            ShipAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::Backward,
            InputMapping::key(KeyCode::ArrowDown, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::TurnLeft,
            InputMapping::key(KeyCode::ArrowLeft, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::TurnRight,
            InputMapping::key(KeyCode::ArrowRight, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::Shoot,
            InputMapping::key(KeyCode::Space, ButtonMode::JustPressed),
        )
        .with_mapping(
            ShipAction::Forward,
            InputMapping::key(KeyCode::ArrowUp, ButtonMode::Pressed),
        )
    }

    fn with_gamepad_mappings(self) -> Self {
        self.with_mapping(
            ShipAction::Forward,
            InputMapping::button(GamepadButton::RightTrigger2, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::Backward,
            InputMapping::button(GamepadButton::LeftTrigger2, ButtonMode::Pressed),
        )
        .with_mapping(
            ShipAction::TurnLeft,
            InputMapping::axis(GamepadAxis::LeftStickX, AxisSide::Negative),
        )
        .with_mapping(
            ShipAction::TurnRight,
            InputMapping::axis(GamepadAxis::LeftStickX, AxisSide::Positive),
        )
        .with_mapping(
            ShipAction::Shoot,
            InputMapping::button(GamepadButton::South, ButtonMode::JustPressed),
        )
    }
}
