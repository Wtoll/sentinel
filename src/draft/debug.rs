//! Debugging tools for the application

use bevy::{color, prelude::*};

use crate::core::app_state::scheduling::GameSystemSet;

/// Plugin for enabling the game's debugging tools
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            draw_gizmos,
            debug_system)
                .in_set(GameSystemSet));
    }
}

/// System for drawing debug gizmos in the world
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X, color::palettes::basic::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y, color::palettes::basic::GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z, color::palettes::basic::BLUE);
}

/// System for debugging things as needed
fn debug_system() {

}