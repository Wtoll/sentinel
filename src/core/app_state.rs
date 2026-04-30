//! Global application state system
//! 

use bevy::prelude::*;

/// Plugin for enabling the game's global application state system
pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .init_state::<PauseState>()
            .configure_sets(Update, (
                scheduling::MenuSystemSet
                    .run_if(in_state(AppState::MainMenu)),
                scheduling::GameSystemSet
                    .run_if(in_state(AppState::InGame)),
                scheduling::RunningSystemSet
                    .in_set(scheduling::GameSystemSet)
                    .run_if(in_state(PauseState::Running)),
                scheduling::PausedSystemSet
                    .in_set(scheduling::GameSystemSet)
                    .run_if(in_state(PauseState::Paused))
            ));
    }
}

/// Whether the application is on the main menu or in game
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// The application is on the main menu
    #[default]
    MainMenu,
    /// The application is in the game
    InGame
}

/// Whether the game is paused or not
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    /// The game is running
    #[default]
    Running,
    /// The game is paused
    Paused
}

/// Scheduling in reference to the global application state
pub mod scheduling {
    use bevy::prelude::*;

    /// A system set that will only run on the main menu
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct MenuSystemSet;

    /// A system set that will only run in game
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct GameSystemSet;

    /// A system set that will run only in game while the game is running
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RunningSystemSet;

    /// A system set that will run only in game while the game is paused
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PausedSystemSet;
}