use bevy::input::gamepad::{GamepadConnection, GamepadEvent};
use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub struct AsteroidInputPlugin;

impl Plugin for AsteroidInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedGamepads::default())
            .add_systems(Update, gamepad_connections)
            .add_systems(Update, input_update_maps.in_set(AsteroidInputSystemSet));
    }
}

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub struct AsteroidInputSystemSet;

pub enum InputActionMode {
    Pressed,
    JustPressed,
}

pub enum AxisSide {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputAction {
    TurnLeft,
    TurnRight,
    Forward,
    Backward,
    Shoot,
}

pub struct InputButtonMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    button: T,
    mode: InputActionMode,
}

pub struct InputAxisMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    axis: T,
    side: AxisSide,
}

pub struct KeyboardInputMap {
    map: HashMap<InputAction, InputButtonMapping<KeyCode>>,
}

impl Default for KeyboardInputMap {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (
                    InputAction::TurnLeft,
                    InputButtonMapping {
                        button: KeyCode::ArrowLeft,
                        mode: InputActionMode::Pressed,
                    },
                ),
                (
                    InputAction::TurnRight,
                    InputButtonMapping {
                        button: KeyCode::ArrowRight,
                        mode: InputActionMode::Pressed,
                    },
                ),
                (
                    InputAction::Forward,
                    InputButtonMapping {
                        button: KeyCode::ArrowUp,
                        mode: InputActionMode::Pressed,
                    },
                ),
                (
                    InputAction::Backward,
                    InputButtonMapping {
                        button: KeyCode::ArrowDown,
                        mode: InputActionMode::Pressed,
                    },
                ),
                (
                    InputAction::Shoot,
                    InputButtonMapping {
                        button: KeyCode::Space,
                        mode: InputActionMode::JustPressed,
                    },
                ),
            ]),
        }
    }
}

pub struct GamepadInputMap {
    button_map: HashMap<InputAction, InputButtonMapping<GamepadButtonType>>,
    axis_map: HashMap<InputAction, InputAxisMapping<GamepadAxisType>>,
}

impl Default for GamepadInputMap {
    fn default() -> Self {
        Self {
            button_map: HashMap::from([(
                InputAction::Shoot,
                InputButtonMapping {
                    button: GamepadButtonType::South,
                    mode: InputActionMode::JustPressed,
                },
            )]),
            axis_map: HashMap::from([
                (
                    InputAction::TurnLeft,
                    InputAxisMapping {
                        axis: GamepadAxisType::LeftStickX,
                        side: AxisSide::Negative,
                    },
                ),
                (
                    InputAction::TurnRight,
                    InputAxisMapping {
                        axis: GamepadAxisType::LeftStickX,
                        side: AxisSide::Positive,
                    },
                ),
                (
                    InputAction::Forward,
                    InputAxisMapping {
                        axis: GamepadAxisType::LeftStickY,
                        side: AxisSide::Positive,
                    },
                ),
                (
                    InputAction::Backward,
                    InputAxisMapping {
                        axis: GamepadAxisType::LeftStickY,
                        side: AxisSide::Negative,
                    },
                ),
            ]),
        }
    }
}

pub struct InputMap {
    keyboard_map: KeyboardInputMap,
    gamepad_map: GamepadInputMap,
    actions: HashMap<InputAction, bool>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            actions: HashMap::from([
                (InputAction::TurnLeft, false),
                (InputAction::TurnRight, false),
                (InputAction::Forward, false),
                (InputAction::Backward, false),
                (InputAction::Shoot, false),
            ]),
            ..default()
        }
    }
}

#[derive(Component, Default)]
pub struct InputController {
    map: InputMap,
}

impl InputController {
    pub fn input_action(&self, action: InputAction) -> bool {
        match self.map.actions.get(&action) {
            Some(value) => *value,
            None => false,
        }
    }
}

fn input_update_maps(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    connected_gamepads: Res<ConnectedGamepads>,
    mut query: Query<&mut InputController>,
) {
    for mut controller in &mut query {
        // Reset everything
        for action_value in controller.map.actions.values_mut() {
            *action_value = false;
        }

        let controller = &mut *controller;

        // Check keyboard
        for (action, mapping) in &controller.map.keyboard_map.map {
            let action_triggered = match mapping.mode {
                InputActionMode::Pressed => keys.pressed(mapping.button),
                InputActionMode::JustPressed => keys.just_pressed(mapping.button),
            };

            let action_value = controller.map.actions.get_mut(action).unwrap();
            *action_value = *action_value || action_triggered;
        }

        // Check gamepads
        if connected_gamepads.gamepads.len() <= 0 {
            return;
        }

        // TODO Handle multiple gamepads for multiple players
        for (action, mapping) in &controller.map.gamepad_map.button_map {
            let gamepad_button = GamepadButton {
                gamepad: connected_gamepads.gamepads[0],
                button_type: mapping.button,
            };

            let action_triggered = match mapping.mode {
                InputActionMode::Pressed => buttons.pressed(gamepad_button),
                InputActionMode::JustPressed => buttons.just_pressed(gamepad_button),
            };

            let action_value = controller.map.actions.get_mut(action).unwrap();
            *action_value = *action_value || action_triggered;
        }

        for (action, mapping) in &controller.map.gamepad_map.axis_map {
            let gamepad_axis = GamepadAxis {
                gamepad: connected_gamepads.gamepads[0],
                axis_type: mapping.axis,
            };

            if let Some(axis_value) = axis.get(gamepad_axis) {
                let action_triggered = match mapping.side {
                    AxisSide::Positive => axis_value > 0.01,
                    AxisSide::Negative => axis_value < 0.01,
                };

                let action_value = controller.map.actions.get_mut(action).unwrap();
                *action_value = *action_value || action_triggered;
            }
        }
    }
}

#[derive(Resource, Default)]
struct ConnectedGamepads {
    gamepads: Vec<Gamepad>,
}

fn gamepad_connections(
    mut connected_gamepads: ResMut<ConnectedGamepads>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for event in evr_gamepad.read() {
        let GamepadEvent::Connection(connection_event) = event else {
            continue;
        };

        match &connection_event.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    connection_event.gamepad, info.name,
                );

                connected_gamepads.gamepads.push(connection_event.gamepad);
            }
            GamepadConnection::Disconnected => {
                debug!(
                    "Lost connection with gamepad: {:?}",
                    connection_event.gamepad
                );

                connected_gamepads
                    .gamepads
                    .retain(|g| *g != connection_event.gamepad);
            }
        }
    }
}
