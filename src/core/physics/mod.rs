use bevy::prelude::*;

mod movement_controller;
pub use movement_controller::MovementController;

use crate::draft::behavior::scheduling::AfterBrainSystemSet;

/// Plugin for managing game physics
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            movement_controller::update_movement_controllers
                .in_set(AfterBrainSystemSet));
    }
}