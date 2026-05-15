//! Input handling systems
//! 


use bevy::prelude::*;

/// Enables input management systems
pub fn plugin(app: &mut App) {
    app.add_plugins(sentinel_input::Plugin);
}
