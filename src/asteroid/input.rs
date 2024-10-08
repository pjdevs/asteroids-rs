use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub enum InputActionMode {
    Pressed,
    JustPressed,
}

#[derive(PartialEq, Eq, Hash)]
pub enum InputAction {
    TurnLeft,
    TurnRight,
    Forward,
    Backward,
    Shoot,
}

pub struct InputMapping<T: Copy + Eq + Hash + Send + Sync + 'static> {
    button: T,
    mode: InputActionMode,
}

#[derive(Resource)]
pub struct InputMap<T: Copy + Eq + Hash + Send + Sync + 'static> {
    map: HashMap<InputAction, InputMapping<T>>,
}

impl<T> InputMap<T>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    fn new() -> Self {
        InputMap {
            map: HashMap::new(),
        }
    }

    pub fn with_mapping(mut self, action: InputAction, mapping: InputMapping<T>) -> Self {
        self.insert_mapping(action, mapping);
        self
    }

    pub fn insert_mapping(&mut self, action: InputAction, mapping: InputMapping<T>) {
        self.map.insert(action, mapping);
    }

    pub fn get_mapping(&self, action: InputAction) -> Option<&InputMapping<T>> {
        self.map.get(&action)
    }

    pub fn input_action(&self, action: InputAction, buttons: &ButtonInput<T>) -> bool {
        let Some(mapping) = self.map.get(&action) else {
            return false;
        };

        let input_function = match mapping.mode {
            InputActionMode::Pressed => ButtonInput::pressed,
            InputActionMode::JustPressed => ButtonInput::just_pressed,
        };

        input_function(buttons, mapping.button)
    }
}

pub struct AsteroidInputPlugin;

impl Plugin for AsteroidInputPlugin {
    fn build(&self, app: &mut App) {
        let default_map = InputMap::new()
            .with_mapping(
                InputAction::TurnLeft,
                InputMapping {
                    button: KeyCode::ArrowLeft,
                    mode: InputActionMode::Pressed,
                },
            )
            .with_mapping(
                InputAction::TurnRight,
                InputMapping {
                    button: KeyCode::ArrowRight,
                    mode: InputActionMode::Pressed,
                },
            )
            .with_mapping(
                InputAction::Forward,
                InputMapping {
                    button: KeyCode::ArrowUp,
                    mode: InputActionMode::Pressed,
                },
            )
            .with_mapping(
                InputAction::Backward,
                InputMapping {
                    button: KeyCode::ArrowDown,
                    mode: InputActionMode::Pressed,
                },
            )
            .with_mapping(
                InputAction::Shoot,
                InputMapping {
                    button: KeyCode::Space,
                    mode: InputActionMode::JustPressed,
                },
            );

        app.insert_resource(default_map);
    }
}
