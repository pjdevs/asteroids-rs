use bevy::input::gamepad::{GamepadConnection, GamepadEvent};
use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

// TODO Make this generic to any custon enum of input action
// TODO Make this support mouse if needed
// TODO Make this support multiple mappings for one action if needed

pub struct AsteroidInputPlugin<A: ActionLike> {
    a: PhantomData<A>
}

impl<A: ActionLike> Plugin for AsteroidInputPlugin<A> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedGamepads::default())
            .add_systems(Update, gamepad_connections)
            .add_systems(Update, input_update_maps::<A>.in_set(AsteroidInputSystemSet));
    }
}

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub struct AsteroidInputSystemSet;

#[derive(Debug)]
pub enum InputActionMode {
    Pressed,
    JustPressed,
}

#[derive(PartialEq, Debug)]
pub enum AxisSide {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum InputAction {
    TurnLeft,
    TurnRight,
    Forward,
    Backward,
    Shoot,
}

#[derive(Debug)]
pub struct InputButtonMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    button: T,
    mode: InputActionMode,
}

#[derive(Debug)]
pub struct InputAxisMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    axis: T,
    side: AxisSide,
}

pub trait ActionLike : Eq + Hash + Send + Sync + 'static {}

#[derive(Default)]
pub struct InputMap<A: ActionLike> {
    keyboard_map: HashMap<InputAction, InputButtonMapping<KeyCode>>,
    gamepad_button_map: HashMap<InputAction, InputButtonMapping<GamepadButtonType>>,
    gamepad_axis_map: HashMap<InputAction, InputAxisMapping<GamepadAxisType>>,
    actions: HashMap<A, bool>,
    associated_gamepad: Option<Gamepad>,
}

#[derive(Component, Default)]
pub struct InputController<A: ActionLike> {
    input_map: InputMap<A>,
}

impl<A: ActionLike> InputController<A> {
    pub fn with_map(input_map: InputMap<A>) -> Self {
        Self {
            input_map
        }
    }

    pub fn input_action(&self, action: A) -> bool {
        match self.input_map.actions.get(&action) {
            Some(value) => *value,
            None => false,
        }
    }
}

fn input_update_maps<A: ActionLike>(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    connected_gamepads: Res<ConnectedGamepads>,
    mut query: Query<&mut InputController<A>>,
) {
    for mut controller in &mut query {
        // Reset everything
        for action_value in controller.input_map.actions.values_mut() {
            *action_value = false;
        }

        let controller = &mut *controller;

        // Check keyboard
        for (action, mapping) in &controller.input_map.keyboard_map.map {
            let action_triggered = match mapping.mode {
                InputActionMode::Pressed => keys.pressed(mapping.button),
                InputActionMode::JustPressed => keys.just_pressed(mapping.button),
            };

            let action_value = controller.input_map.actions.get_mut(action).unwrap();
            *action_value = *action_value || action_triggered;
        }

        // Check gamepads
        if connected_gamepads.gamepads.len() <= 0 {
            return;
        }

        // TODO Handle multiple gamepads for multiple players (by associating a gamepad/keyboard id to each controller)
        let controller_gamepad = connected_gamepads.gamepads[0];

        for (action, mapping) in &controller.input_map.gamepad_map.button_map {
            let gamepad_button = GamepadButton {
                gamepad: controller_gamepad,
                button_type: mapping.button,
            };

            let action_triggered = match mapping.mode {
                InputActionMode::Pressed => buttons.pressed(gamepad_button),
                InputActionMode::JustPressed => buttons.just_pressed(gamepad_button),
            };

            let action_value = controller.input_map.actions.get_mut(action).unwrap();
            *action_value = *action_value || action_triggered;
        }

        for (action, mapping) in &controller.input_map.gamepad_map.axis_map {
            let gamepad_axis = GamepadAxis {
                gamepad: controller_gamepad,
                axis_type: mapping.axis,
            };

            if let Some(axis_value) = axis.get(gamepad_axis) {
                let action_triggered = match mapping.side {
                    AxisSide::Positive => axis_value > 0.5,
                    AxisSide::Negative => axis_value < -0.5,
                };

                let action_value = controller.input_map.actions.get_mut(action).unwrap();
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
