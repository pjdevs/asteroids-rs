use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn player_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let Ok((player, mut player_transform)) = player_query.get_single_mut() else {
        return;
    };

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
        (input_direction * 5.).extend(0.0);
}
