#![forbid(missing_docs)]
//! A debug inspector for Bevy
//! 

use bevy::app::{App, Plugin};

/// Plugin for enabling the debug inspector
#[derive(Default)]
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        
    }
}