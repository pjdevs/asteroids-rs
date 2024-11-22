use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenuLoading,
    MainMenu,
    GameLoading,
    Game,
}

// #[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
// pub enum PauseState {
//     #[default]
//     Running,
//     Paused,
// }
