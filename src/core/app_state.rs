use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
            .init_state::<PauseState>()
            .configure_sets(Update, (
                scheduling::MenuSystemSet
                    .run_if(in_state(MenuState::MainMenu)),
                scheduling::GameSystemSet
                    .run_if(in_state(MenuState::InGame)),
                scheduling::RunningSystemSet
                    .in_set(scheduling::GameSystemSet)
                    .run_if(in_state(PauseState::Running)),
                scheduling::PausedSystemSet
                    .in_set(scheduling::GameSystemSet)
                    .run_if(in_state(PauseState::Paused))
            ));
    }
}

/// Enum for the main menu state
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    #[default]
    MainMenu,
    InGame
}

/// Enum for the pause state
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Running,
    Paused
}

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