//! Progression system
//! 

use bevy::prelude::*;

use crate::core::state::GameState;

/// Plugin for progression
pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Entering), make_graph);
}



fn make_graph() {

}