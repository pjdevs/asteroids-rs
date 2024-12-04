use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Default)]
pub struct InputPlugin<A: ActionLike> {
    a: PhantomData<A>,
}

impl<A: ActionLike> Plugin for InputPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                input_gamepad_list,
                input_update_maps::<A>.in_set(InputSystem::UpdateInput),
            ),
        );
    }
}

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum InputSystem {
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
    GamepadButton(InputButtonMapping<GamepadButton>),
    GamepadAxis(InputAxisMapping<GamepadAxis>),
}

impl InputMapping {
    pub fn key(key: KeyCode, mode: ButtonMode) -> Self {
        Self::KeyboardButton(InputButtonMapping { button: key, mode })
    }

    pub fn button(button: GamepadButton, mode: ButtonMode) -> Self {
        Self::GamepadButton(InputButtonMapping { button, mode })
    }

    pub fn axis(axis: GamepadAxis, side: AxisSide) -> Self {
        Self::GamepadAxis(InputAxisMapping { axis, side })
    }

    fn is_triggered(
        &self,
        keys: &ButtonInput<KeyCode>,
        associated_gamepad: Option<&Gamepad>,
    ) -> bool {
        match self {
            InputMapping::KeyboardButton(keyboard_mapping) => match keyboard_mapping.mode {
                ButtonMode::Pressed => keys.pressed(keyboard_mapping.button),
                ButtonMode::JustPressed => keys.just_pressed(keyboard_mapping.button),
            },
            InputMapping::GamepadButton(gamepad_mapping) if associated_gamepad.is_some() => {
                match gamepad_mapping.mode {
                    ButtonMode::Pressed => {
                        associated_gamepad.unwrap().pressed(gamepad_mapping.button)
                    }
                    ButtonMode::JustPressed => associated_gamepad
                        .unwrap()
                        .just_pressed(gamepad_mapping.button),
                }
            }
            InputMapping::GamepadAxis(gamepad_mapping) if associated_gamepad.is_some() => {
                match associated_gamepad.unwrap().get(gamepad_mapping.axis) {
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

#[derive(Component, Default)]
pub struct InputMap<A: ActionLike> {
    action_map: HashMap<A, Vec<InputMapping>>,
    action_values: HashMap<A, bool>,
    associated_gamepad: Option<Entity>,
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

    pub fn with_gamepad(mut self, gamepad: Option<Entity>) -> Self {
        self.associated_gamepad = gamepad;
        self
    }

    pub fn input_action(&self, action: A) -> bool {
        match self.action_values.get(&action) {
            Some(value) => *value,
            None => false,
        }
    }
}

fn input_update_maps<A: ActionLike>(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads_query: Query<&Gamepad>,
    mut map_query: Query<&mut InputMap<A>>,
) {
    for mut map in &mut map_query {
        // Reset values
        for action_value in map.action_values.values_mut() {
            *action_value = false;
        }

        // Update inputs
        let map = &mut *map;
        for (action, mappings) in &mut map.action_map {
            let action_triggered = mappings.iter().fold(false, |triggered, mapping| {
                let gamepad = map
                    .associated_gamepad
                    .and_then(|gamepad_entity| gamepads_query.get(gamepad_entity).ok());
                triggered || mapping.is_triggered(&keys, gamepad)
            });

            map.action_values.insert(*action, action_triggered);
        }
    }
}

pub fn any_gamepad_connected() -> impl FnMut(Query<&Gamepad>) -> bool + Clone {
    move |gamepads: Query<&Gamepad>| !gamepads.is_empty()
}

pub fn on_gamepad_connection() -> impl FnMut(EventReader<GamepadConnectionEvent>) -> bool + Clone {
    move |mut reader: EventReader<GamepadConnectionEvent>| reader.read().any(|e| e.connected())
}

// pub fn on_gamepad_disconnection(
//     gamepad_id: usize,
// ) -> impl FnMut(EventReader<GamepadConnectionEvent>) -> bool + Clone {
//     move |mut reader: EventReader<GamepadConnectionEvent>| {
//         reader
//             .read()
//             .any(|e| e.gamepad.id == gamepad_id && e.disconnected())
//     }
// }

#[derive(Resource, Deref, DerefMut)]
pub struct Gamepads(HashSet<Entity>);

fn input_gamepad_list(
    mut events: EventReader<GamepadConnectionEvent>,
    mut gamepads: ResMut<Gamepads>,
) {
    for event in events.read() {
        match &event.connection {
            GamepadConnection::Connected {
                name: _,
                vendor_id: _,
                product_id: _,
            } => {
                gamepads.insert(event.gamepad);
            }
            GamepadConnection::Disconnected => {
                gamepads.remove(&event.gamepad);
            }
        }
    }
}

pub mod prelude {
    pub use super::*;
}
