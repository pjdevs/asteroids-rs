use bevy::prelude::*;

use super::{
    input::{AsteroidInputSystemSet, InputAction, InputController},
    physics::Movement,
};

pub struct AsteroidControllerPlugin;

impl Plugin for AsteroidControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            controller_keyboard_input_system.after(AsteroidInputSystemSet),
        );
    }
}

#[derive(Component, Default)]
pub struct Speed {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

fn controller_keyboard_input_system(mut query: Query<(&mut Movement, &Speed, &InputController)>) {
    for (mut movement, speed, controller) in &mut query {
        controller_move(&mut movement, speed, controller);
    }
}

fn controller_move(movement: &mut Movement, speed: &Speed, controller: &InputController) {
    let mut input_direction = Vec2::ZERO;

    if controller.input_action(InputAction::Forward) {
        input_direction.y += 1.0;
    }

    if controller.input_action(InputAction::Backward) {
        input_direction.y -= 1.0;
    }

    if controller.input_action(InputAction::TurnLeft) {
        input_direction.x -= 1.0;
    }

    if controller.input_action(InputAction::TurnRight) {
        input_direction.x += 1.0;
    }

    // Rotation
    movement.angular_velocity = -input_direction.x * speed.rotation_speed;

    // Translation
    movement.acceleration = movement.get_direction() * speed.movement_speed * input_direction.y;
}
