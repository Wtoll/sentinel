//! Physics

use bevy::prelude::*;

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
