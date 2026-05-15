//! A sandbox for quickly writing debugging code
//! 

use bevy::prelude::*;

use crate::core::state::AppState;

/// A plugin that enables a sandbox for quickly writing debugging code
pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, sandbox)
            .add_systems(Update, skip_main_menu
                .run_if(in_state(AppState::MainMenu)));
    }
}



fn skip_main_menu(
    mut app_state: ResMut<NextState<AppState>>
) {
    app_state.set(AppState::Game);
}


fn sandbox(
) {

    // debug!("Sandbox");

}