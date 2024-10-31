use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AsteroidGameState {
    #[default]
    MainMenuLoading,
    MainMenu,
    GameLoading,
    Game,
}

// #[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
// pub enum AsteroidPauseState {
//     #[default]
//     Running,
//     Paused,
// }
