use bevy::prelude::*;

use super::Movement;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Movement)>,
) {
    let (player, mut player_movement) = player_query.single_mut();
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
    player_movement.velocity = input_direction * player.speed;
}
