use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AsteroidGameState {
    #[default]
    MainMenu,
    GameLoadingScreen,
    InGame,
}

// #[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
// pub enum AsteroidPauseState {
//     #[default]
//     Running,
//     Paused,
// }
