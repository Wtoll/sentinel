//! A sandbox for quickly writing debugging code
//! 

use bevy::{color, prelude::*};

use crate::core::state::AppState;

/// A plugin that enables a sandbox for quickly writing debugging code
pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, sandbox)
            .add_systems(Update, draw_gizmos)
            .add_systems(Update, skip_main_menu
                .run_if(in_state(AppState::MainMenu)));
    }
}



fn skip_main_menu(
    mut app_state: ResMut<NextState<AppState>>
) {
    app_state.set(AppState::Game);
}

/// System for drawing debug gizmos in the world
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X, color::palettes::basic::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y, color::palettes::basic::GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z, color::palettes::basic::BLUE);
}


fn sandbox(
) {

    // debug!("Sandbox");

}