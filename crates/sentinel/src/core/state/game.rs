//! Game state manager
//! 

#[cfg(feature = "debug")]
use strum::EnumIter;

use bevy::prelude::*;

use super::AppState;

pub fn plugin(app: &mut App) {
    app
        .add_sub_state::<GameState>()
        .add_message::<GameLeavingTarget>()
        .add_systems(PreUpdate, entering::poll_state
            .run_if(in_state(GameState::Entering)))
        .add_systems(PreUpdate, leaving::poll_state
            .run_if(in_state(GameState::Leaving)))
        .add_systems(PreUpdate, loading::poll_state
            .run_if(in_state(GameState::Loading)));
}

/// The state of the game
/// 
/// This is a sub-state of [`AppState::Game`]
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug", derive(EnumIter))]
#[source(AppState = AppState::Game)]
pub enum GameState {
    /// Entering the game
    #[default]
    Entering,
    /// The game is running
    Running,
    /// The game is paused
    Paused,
    /// Within an in-game menu
    InGameMenu,
    /// Loading within the game
    Loading,
    /// Death screen
    Died,
    /// Leaving the game
    Leaving
}

mod entering {
    use bevy::prelude::*;

    use crate::core::state::GameState;

    /// A component that will block leaving
    /// [`GameState::Entering`](super::GameState::Entering) while it exists on
    /// an entity.
    #[derive(Component, Default)]
    pub struct GameEnteringTask;

    pub fn poll_state(
        task_query: Query<&GameEnteringTask>,
        mut game_state: ResMut<NextState<GameState>>
    ) {
        if task_query.count() == 0 {
            game_state.set(GameState::Running);
        }
    }
}

pub use entering::GameEnteringTask;

mod leaving {
    use bevy::prelude::*;

    use crate::core::state::AppState;

    /// The [`AppState`] the game is leaving to.
    /// 
    /// The default behavior is for the game to make a full exit from the
    /// application. Any system can send a signal, however, to exit to the main
    /// menu instead. If the main menu becomes the target, all further messages
    /// are entirely ignored.
    #[derive(Message, Default, Clone, Copy, PartialEq, Eq)]
    pub enum GameLeavingTarget {
        /// The game is leaving to the main menu
        MainMenu,
        /// The game is leaving to exit the application
        #[default]
        Exit
    }

    /// A component that will block leaving
    /// [`GameState::Leaving`](super::GameState::Leaving) while it exists on an
    /// entity.
    #[derive(Component, Default)]
    pub struct GameLeavingTask;

    pub fn poll_state(
        task_query: Query<&GameLeavingTask>,
        mut app_state: ResMut<NextState<AppState>>,
        mut target: Local<GameLeavingTarget>,
        mut target_messages: MessageReader<GameLeavingTarget>
    ) {
        if *target != GameLeavingTarget::MainMenu {
            for message in target_messages.read() {
                if *message == GameLeavingTarget::MainMenu {
                    *target = *message;
                }
            }
        }
        
        if task_query.count() == 0 {
            app_state.set(match *target {
                GameLeavingTarget::MainMenu => AppState::MainMenu,
                GameLeavingTarget::Exit => AppState::Exiting,
            })
        }
    }
}

pub use leaving::{GameLeavingTarget, GameLeavingTask};

mod loading {
    use bevy::prelude::*;

    use super::GameState;

    /// A component that will block leaving
    /// [`GameState::Loading`](super::GameState::Loading) while it exists on an
    /// entity.
    #[derive(Component, Default)]
    pub struct GameLoadingTask;

    pub fn poll_state(
        task_query: Query<&GameLoadingTask>,
        mut game_state: ResMut<NextState<GameState>>
    ) {
        if task_query.count() == 0 {
            game_state.set(GameState::Running);
        }
    }
}

pub use loading::GameLoadingTask;
