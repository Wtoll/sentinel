use bevy::prelude::*;

use crate::core::AppState;

pub fn plugin(app: &mut App) {
    app
        .add_sub_state::<GameState>()
        .add_systems(Update, running::keybinds
            .run_if(in_state(GameState::Running)))
        .add_systems(Update, paused::keybinds
            .run_if(in_state(GameState::Paused)));
}

/// The state of the game
/// 
/// This is a sub-state of `AppState::InGame`
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]
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

mod running {
    use bevy::prelude::*;

    use leafwing_input_manager::prelude::*;

    use crate::core::{GameState, input::GameAction};

    /// System for handling keybinds while the game is running
    pub fn keybinds(
        mut next_state: ResMut<NextState<GameState>>,
        action_states: Query<&ActionState<GameAction>>
    ) {
        for state in action_states {
            if state.just_pressed(&GameAction::Pause) {
                next_state.set(GameState::Paused);
            }
        }
    }
}

mod paused {
    use bevy::prelude::*;

    use leafwing_input_manager::prelude::*;

    use crate::core::{GameState, input::GameAction};

    /// System for handling keybinds while the game is paused
    pub fn keybinds(
        mut next_state: ResMut<NextState<GameState>>,
        action_states: Query<&ActionState<GameAction>>
    ) {
        for state in action_states {
            if state.just_pressed(&GameAction::Pause) {
                next_state.set(GameState::Running);
            }
        }
    }
}

