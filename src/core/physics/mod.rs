//! Physics

use bevy::prelude::*;

mod movement_controller;
pub use movement_controller::MovementController;

use crate::draft::behavior::scheduling::BrainSystemSet;

/// Plugin for managing game physics
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            movement_controller::update_movement_controllers
                .after(BrainSystemSet));
    }
}






/// The velocity (linear and angular) of an entity
/// 
/// Angular velocity is assumed to be body centered
#[derive(Component, Default)]
pub struct Velocity {
    /// The linear velocity of the body
    pub linear: Vec3,
    /// The angular velocity of the body
    pub angular: Vec3
}




/// A collision box for an entity
#[derive(Component, Default)]
pub struct Collider {

}