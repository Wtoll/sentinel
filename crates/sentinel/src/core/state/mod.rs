//! Handles the global state of the application
//! 

#[cfg(feature = "debug")]
use strum::EnumIter;

use bevy::prelude::*;

mod game;
pub use game::{GameState, GameEnteringTask, GameLeavingTarget, GameLeavingTask, GameLoadingTask};

/// Plugin for enabling the game's global state manager
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(game::plugin)
            .init_state::<AppState>()
            .add_systems(PreUpdate, entering::poll_state
                .run_if(in_state(AppState::Entering)))
            .add_systems(PreUpdate, exiting::poll_state
                .run_if(in_state(AppState::Exiting)));
    }
}

/// The global state of the application
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(EnumIter))]
pub enum AppState {
    /// The application is starting up
    #[default]
    Entering,
    /// The application is in the main menu
    MainMenu,
    /// The application is in the game
    Game,
    /// The application is exiting
    Exiting
}

mod entering {
    use bevy::prelude::*;

    use super::AppState;

    /// A component that will block leaving
    /// [`AppState::Entering`](super::AppState::Entering) while it exists on an
    /// entity.
    #[derive(Component, Default)]
    pub struct AppEnteringTask;

    /// Moves into [`AppState::MainMenu`] when [`Entering`](AppState::Entering)
    /// is complete.
    pub fn poll_state(
        task_query: Query<&AppEnteringTask>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        if task_query.count() == 0 {
            app_state.set(AppState::MainMenu);
        }
    }
}

pub use entering::AppEnteringTask;

mod exiting {
    use bevy::prelude::*;

    /// A component that will block leaving
    /// [`AppState::Exiting`](super::AppState::Exiting) while it exists on an
    /// entity.
    #[derive(Component, Default)]
    pub struct AppExitingTask;

    /// Sends an exit signal to the program when
    /// [`AppState::Exiting`](super::AppState::Exiting) is complete.
    pub fn poll_state(
        task_query: Query<&AppExitingTask>,
        mut writer: MessageWriter<AppExit>,
    ) {
        if task_query.count() == 0 {
            writer.write(AppExit::Success);
        }
    }
}

pub use exiting::AppExitingTask;