use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

// TODO Make this support mouse if needed

#[derive(Default)]
pub struct AsteroidInputPlugin<A: ActionLike> {
    a: PhantomData<A>,
}

impl<A: ActionLike> Plugin for AsteroidInputPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            input_update_maps::<A>.in_set(AsteroidInputSystem::UpdateInput),
        );
    }
}

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidInputSystem {
    UpdateInput,
}

#[derive(Debug)]
pub enum ButtonMode {
    Pressed,
    JustPressed,
}

#[derive(PartialEq, Debug)]
pub enum AxisSide {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct InputButtonMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    button: T,
    mode: ButtonMode,
}

#[derive(Debug)]
pub struct InputAxisMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    axis: T,
    side: AxisSide,
}

pub enum InputMapping {
    KeyboardButton(InputButtonMapping<KeyCode>),
    GamepadButton(InputButtonMapping<GamepadButtonType>),
    GamepadAxis(InputAxisMapping<GamepadAxisType>),
}

impl InputMapping {
    pub fn key(key: KeyCode, mode: ButtonMode) -> Self {
        Self::KeyboardButton(InputButtonMapping { button: key, mode })
    }

    pub fn button(button: GamepadButtonType, mode: ButtonMode) -> Self {
        Self::GamepadButton(InputButtonMapping { button, mode })
    }

    pub fn axis(axis: GamepadAxisType, side: AxisSide) -> Self {
        Self::GamepadAxis(InputAxisMapping { axis, side })
    }

    fn is_triggered(
        &self,
        keys: &ButtonInput<KeyCode>,
        buttons: &ButtonInput<GamepadButton>,
        axis: &Axis<GamepadAxis>,
        connected_gamepads: &Gamepads,
        associated_gamepad: Option<Gamepad>,
    ) -> bool {
        let use_gamepad =
            associated_gamepad.is_some_and(|gamepad| connected_gamepads.contains(gamepad));

        match self {
            InputMapping::KeyboardButton(keyboard_mapping) => match keyboard_mapping.mode {
                ButtonMode::Pressed => keys.pressed(keyboard_mapping.button),
                ButtonMode::JustPressed => keys.just_pressed(keyboard_mapping.button),
            },
            InputMapping::GamepadButton(gamepad_mapping) if use_gamepad => {
                let gamepad_button = GamepadButton {
                    gamepad: associated_gamepad.unwrap(),
                    button_type: gamepad_mapping.button,
                };

                match gamepad_mapping.mode {
                    ButtonMode::Pressed => buttons.pressed(gamepad_button),
                    ButtonMode::JustPressed => buttons.just_pressed(gamepad_button),
                }
            }
            InputMapping::GamepadAxis(gamepad_mapping) if use_gamepad => {
                let gamepad_axis = GamepadAxis {
                    gamepad: associated_gamepad.unwrap(),
                    axis_type: gamepad_mapping.axis,
                };

                match axis.get(gamepad_axis) {
                    Some(axis_value) => match gamepad_mapping.side {
                        AxisSide::Positive => axis_value > 0.5,
                        AxisSide::Negative => axis_value < -0.5,
                    },
                    None => false,
                }
            }
            _ => false,
        }
    }
}

pub trait ActionLike: Default + Copy + Eq + Hash + Send + Sync + 'static {}

#[derive(Default)]
pub struct InputMap<A: ActionLike> {
    action_map: HashMap<A, Vec<InputMapping>>,
    action_values: HashMap<A, bool>,
    associated_gamepad: Option<Gamepad>,
}

impl<A: ActionLike> InputMap<A> {
    pub fn with_mapping(mut self, action: A, mapping: InputMapping) -> Self {
        match self.action_map.get_mut(&action) {
            Some(mappings) => {
                mappings.push(mapping);
            }
            None => {
                self.action_map.insert(action, vec![mapping]);
            }
        };
        self
    }

    pub fn with_gamepad(mut self, gamepad: Gamepad) -> Self {
        self.associated_gamepad = Some(gamepad);
        self
    }

    fn input_action(&self, action: A) -> bool {
        match self.action_values.get(&action) {
            Some(value) => *value,
            None => false,
        }
    }

    fn update(
        &mut self,
        keys: &ButtonInput<KeyCode>,
        buttons: &ButtonInput<GamepadButton>,
        axis: &Axis<GamepadAxis>,
        connected_gamepads: &Gamepads,
    ) {
        // Reset values
        for action_value in self.action_values.values_mut() {
            *action_value = false;
        }

        for (action, mappings) in &self.action_map {
            let action_triggered = mappings.iter().fold(false, |triggered, mapping| {
                triggered
                    || mapping.is_triggered(
                        keys,
                        buttons,
                        axis,
                        connected_gamepads,
                        self.associated_gamepad,
                    )
            });

            self.action_values.insert(*action, action_triggered);
        }
    }
}

#[derive(Component, Default)]
pub struct InputController<A: ActionLike> {
    input_map: InputMap<A>,
}

impl<A: ActionLike> InputController<A> {
    pub fn from_map(input_map: InputMap<A>) -> Self {
        Self { input_map }
    }

    pub fn input_action(&self, action: A) -> bool {
        self.input_map.input_action(action)
    }

    fn update(
        &mut self,
        keys: &ButtonInput<KeyCode>,
        buttons: &ButtonInput<GamepadButton>,
        axis: &Axis<GamepadAxis>,
        connected_gamepads: &Gamepads,
    ) {
        self.input_map
            .update(keys, buttons, axis, connected_gamepads);
    }
}

fn input_update_maps<A: ActionLike>(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    connected_gamepads: Res<Gamepads>,
    mut query: Query<&mut InputController<A>>,
) {
    for mut controller in &mut query {
        controller.update(&keys, &buttons, &axis, &connected_gamepads);
    }
}

// pub fn on_gamepad_connection(
//     gamepad_id: usize,
// ) -> impl FnMut(EventReader<GamepadConnectionEvent>) -> bool + Clone {
//     move |mut reader: EventReader<GamepadConnectionEvent>| {
//         reader
//             .read()
//             .any(|e| e.gamepad.id == gamepad_id && e.connected())
//     }
// }

// pub fn on_gamepad_disconnection(
//     gamepad_id: usize,
// ) -> impl FnMut(EventReader<GamepadConnectionEvent>) -> bool + Clone {
//     move |mut reader: EventReader<GamepadConnectionEvent>| {
//         reader
//             .read()
//             .any(|e| e.gamepad.id == gamepad_id && e.disconnected())
//     }
// }

pub mod prelude {
    pub use super::*;
}
