//! Progression system
//! 
//! 

use bevy::prelude::*;

/// Plugin for managing game progression
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}





fn startup() {
    
}


fn update() {

}