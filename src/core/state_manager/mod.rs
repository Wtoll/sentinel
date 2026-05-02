//! Global application state manager
//! 

use bevy::prelude::*;

/// Plugin for enabling the game's global state manager
pub struct StateManagerPlugin;

impl Plugin for StateManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(game::plugin)
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::Exit), close_from_exit);
    }
}

/// The state of the application
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// The application is on the main menu
    #[default]
    MainMenu,
    /// The application is in the game
    InGame,
    /// The application is about to exit
    Exit
}

/// System that initiates a close when the application enters the
/// `AppState::Exit` state
fn close_from_exit(
    mut writer: MessageWriter<AppExit>
) {
    writer.write(AppExit::Success);
}

mod game;
pub use game::GameState;