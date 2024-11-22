use crate::asset;
use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::physics::prelude::*;
use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>().add_systems(
            Update,
            ((ship_move_system, ship_shoot_system)
                .run_if(in_state(GameState::Game))
                .in_set(ShipSystem::UpdateShips),),
        );
    }
}

// Events

#[derive(Event)]
pub struct ShootEvent;

// Components

#[derive(Component, Default)]
pub struct ShipMovement {
    pub direction: Vec2,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct ShipShoot {
    pub shoot: bool,
    pub projectile_speed: f32,
    pub projectile_texture: Handle<Image>,
    pub projectile_size: Handle<SizeAsset>,
    pub projectile_color: Color,
    pub projectile_members: LayerMask,
    pub projectile_filters: LayerMask,
}

impl Default for ShipShoot {
    fn default() -> Self {
        Self {
            shoot: false,
            projectile_speed: 600.0,
            projectile_texture: Default::default(),
            projectile_size: Default::default(),
            projectile_color: Color::srgb(5.0, 5.0, 7.0),
            projectile_members: layers::PLAYER_MASK,
            projectile_filters: layers::ENEMY_MASK,
        }
    }
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum ShipSystem {
    UpdateShips,
}

fn ship_shoot_system(
    mut commands: Commands,
    mut shoot_events: EventWriter<ShootEvent>,
    sizes: Res<Assets<SizeAsset>>,
    player_query: Query<(&Movement, &ShipShoot)>,
) {
    for (movement, shoot) in &player_query {
        if shoot.shoot {
            let size_asset = asset!(sizes, &shoot.projectile_size);

            commands.spawn((
                ProjectileBundle {
                    sprite: SpriteBundle {
                        texture: shoot.projectile_texture.clone_weak(),
                        sprite: Sprite {
                            custom_size: Some(size_asset.sprite_size),
                            color: shoot.projectile_color,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    movement: Movement {
                        position: movement.position,
                        velocity: movement.get_direction() * shoot.projectile_speed,
                        rotation: movement.rotation,
                        ..Default::default()
                    },
                    collider: Collider::from_shape(Shape::Obb(Obb2d::new(
                        Vec2::ZERO,
                        size_asset.collider_size / 2.0,
                        0.0,
                    ))),
                    layers: CollisionLayers::new(
                        shoot.projectile_members,
                        shoot.projectile_filters,
                    ),
                    damager: Damager::Constant(50).into(),
                    ..Default::default()
                },
                #[cfg(feature = "dev")]
                Name::new("Projectile"),
            ));

            shoot_events.send(ShootEvent);
        }
    }
}

fn ship_move_system(mut query: Query<(&mut Movement, &ShipMovement)>) {
    for (mut movement, ship) in &mut query {
        // Rotation
        movement.angular_velocity = -ship.direction.x * ship.rotation_speed;

        // Translation
        movement.acceleration = movement.get_direction() * ship.movement_speed * ship.direction.y;
    }
}
